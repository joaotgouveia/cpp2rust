extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct context_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct context_wchar_t_ {}
pub type status = u16;
pub const status_foo: status = 43690;
pub const status_bar: status = 21845;
pub const status_foobar: status = 43605;
pub fn operator_cmp_0(lhs: Ptr<parse_call_validator>, rhs: Ptr<parse_call_validator>) -> () {
    return std::cmp::Ord::cmp(&(lhs), &(rhs));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct parse_call_validator_parse_function_not_called {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct parse_call_validator {}
impl std::cmp::Ord for parse_call_validator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_0(
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
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
        {
            operator_cmp_0(
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
                Rc::new(RefCell::new(parse_call_validator {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub fn get_format_types_1() -> () {
    return unimplemented!();
}
pub type execution_modus = ();
pub const execution_modus_partial: execution_modus = 0;
pub const execution_modus_full: execution_modus = 1;
pub fn invalid_types_2(valid: ()) -> () {
    let valid: Value<()> = Rc::new(RefCell::new(valid));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let types: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for type_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut type_: () = (type_.read());
        if (unimplemented!() != unimplemented!()) {
            continue 'loop_;
        }
        switch!(match type_ {
            __v if __v == b"a"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"A"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"b"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"B"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"c"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"d"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"e"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"E"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"f"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"F"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"g"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"G"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"o"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"p"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"P"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"s"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"x"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"X"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"?"[(0_()) as usize] => {
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
pub fn invalid_types_3(valid: ()) -> () {
    let valid: Value<()> = Rc::new(RefCell::new(valid));
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let types: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for type_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut type_: () = (type_.read());
        if (unimplemented!() != unimplemented!()) {
            continue 'loop_;
        }
        switch!(match type_ {
            __v if __v == b"a"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"A"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"b"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"B"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"c"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"d"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"e"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"E"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"f"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"F"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"g"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"G"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"o"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"p"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"P"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"s"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"x"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"X"[(0_()) as usize] => {
                unimplemented!();
                break;
            }
            __v if __v == b"?"[(0_()) as usize] => {
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
pub fn format_test_handle_4(check: FnPtr<Generic>, check_exception: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    let check_exception: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check_exception));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_0.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foo));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_1.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foo));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_2.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foo));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_3.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foo));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_4.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_bar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_5.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_bar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_6.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_bar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_7: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_7.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_bar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_8.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foobar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_9.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foobar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_10.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foobar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__status_::new({ __tmp_11.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foobar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(29).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    let __tmp_12: Value<()> = Rc::new(RefCell::new(({ invalid_types_2(unimplemented!()) })));
    let __range1: Ptr<()> = __tmp_12.as_pointer();
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let fmt: Ptr<()> = unimplemented!();
        ({
            let _arg0: () = unimplemented!();
            let _arg1: () = unimplemented!();
            let _arg2: Value<status> = Rc::new(RefCell::new(status_foo));
            (*check_exception.borrow())
                .spec::<fn((), (), Ptr<status>)>(2)
                .call(_arg0, _arg1, _arg2.as_pointer())
        });
    }
}
pub fn format_test_handle_5(check: FnPtr<Generic>, check_exception: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    let check_exception: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check_exception));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_13: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_13.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foo));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_14: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_14.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foo));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_15: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_15.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foo));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_16: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_16.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foo));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_17: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_17.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_bar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_18: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_18.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_bar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_19: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_19.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_bar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_20: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_20.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_bar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_21: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_21.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foobar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_22: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_22.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foobar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_23: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_23.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foobar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_24: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__status_::new({ __tmp_24.as_pointer() })
        };
        let _args: Value<status> = Rc::new(RefCell::new(status_foobar));
        (*check.borrow()).spec::<fn((), (), Ptr<status>)>(60).call(
            _expected,
            _fmt,
            _args.as_pointer(),
        )
    });
    let __tmp_25: Value<()> = Rc::new(RefCell::new(({ invalid_types_3(unimplemented!()) })));
    let __range1: Ptr<()> = __tmp_25.as_pointer();
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let fmt: Ptr<()> = unimplemented!();
        ({
            let _arg1: () = unimplemented!();
            let _arg2: Value<status> = Rc::new(RefCell::new(status_foo));
            (*check_exception.borrow())
                .spec::<fn((), (), Ptr<status>)>(5)
                .call(unimplemented!(), _arg1, _arg2.as_pointer())
        });
    }
}
pub fn format_tests_6(check: FnPtr<Generic>, check_exception: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    let check_exception: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check_exception));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_26: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char_::new({ __tmp_26.as_pointer() })
        };
        (*check.borrow())
            .spec::<fn((), ())>(0)
            .call(_expected, _fmt)
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_27: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char_::new({ __tmp_27.as_pointer() })
        };
        (*check.borrow())
            .spec::<fn((), ())>(0)
            .call(_expected, _fmt)
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_28: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char_::new({ __tmp_28.as_pointer() })
        };
        (*check.borrow())
            .spec::<fn((), ())>(0)
            .call(_expected, _fmt)
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_29: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__char_::new({ __tmp_29.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new((' ' as ())));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_30: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char_::new({ __tmp_30.as_pointer() })
        };
        (*check.borrow())
            .spec::<fn((), ())>(0)
            .call(_expected, _fmt)
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_31: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__char_::new({ __tmp_31.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new((' ' as ())));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_32: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__bool__bool_::new({ __tmp_32.as_pointer() })
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(false));
        let _args_3: Value<()> = Rc::new(RefCell::new(true));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>)>(2)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_33: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__bool__bool_::new({ __tmp_33.as_pointer() })
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(false));
        let _args_3: Value<()> = Rc::new(RefCell::new(true));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>)>(2)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_34: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ :: new ( { __tmp_34.as_pointer() } , )
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(1_()));
        let _args_3: Value<()> = Rc::new(RefCell::new(2_()));
        let _args_4: Value<()> = Rc::new(RefCell::new(3_()));
        let _args_5: Value<()> = Rc::new(RefCell::new(4_()));
        let _args_6: Value<()> = Rc::new(RefCell::new(5_()));
        let _args_7: Value<()> = Rc::new(RefCell::new(6_()));
        let _args_8: Value<()> = Rc::new(RefCell::new(7_()));
        let _args_9: Value<()> = Rc::new(RefCell::new(8_()));
        let _args_10: Value<()> = Rc::new(RefCell::new(9_()));
        let _args_11: Value<()> = Rc::new(RefCell::new(0_()));
        let _args_12: Value<()> = Rc::new(RefCell::new(1_()));
        let _args_13: Value<()> = Rc::new(RefCell::new(2_()));
        let _args_14: Value<()> = Rc::new(RefCell::new(3_()));
        let _args_15: Value<()> = Rc::new(RefCell::new(4_()));
        let _args_16: Value<()> = Rc::new(RefCell::new(5_()));
        let _args_17: Value<()> = Rc::new(RefCell::new(6_()));
        let _args_18: Value<()> = Rc::new(RefCell::new(7_()));
        let _args_19: Value<()> = Rc::new(RefCell::new(8_()));
        let _args_20: Value<()> = Rc::new(RefCell::new(9_()));
        let _args_21: Value<()> = Rc::new(RefCell::new(0_()));
        (*check.borrow())
            .spec::<fn(
                (),
                (),
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
            )>(3)
            .call(
                _expected,
                _fmt,
                _args_2.as_pointer(),
                _args_3.as_pointer(),
                _args_4.as_pointer(),
                _args_5.as_pointer(),
                _args_6.as_pointer(),
                _args_7.as_pointer(),
                _args_8.as_pointer(),
                _args_9.as_pointer(),
                _args_10.as_pointer(),
                _args_11.as_pointer(),
                _args_12.as_pointer(),
                _args_13.as_pointer(),
                _args_14.as_pointer(),
                _args_15.as_pointer(),
                _args_16.as_pointer(),
                _args_17.as_pointer(),
                _args_18.as_pointer(),
                _args_19.as_pointer(),
                _args_20.as_pointer(),
                _args_21.as_pointer(),
            )
    });
    {}
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_35: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__char__char__char__char__char__char__char_::new({
                __tmp_35.as_pointer()
            })
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(('0' as ())));
        let _args_3: Value<()> = Rc::new(RefCell::new(('9' as ())));
        let _args_4: Value<()> = Rc::new(RefCell::new(('a' as ())));
        let _args_5: Value<()> = Rc::new(RefCell::new(('z' as ())));
        let _args_6: Value<()> = Rc::new(RefCell::new(('A' as ())));
        let _args_7: Value<()> = Rc::new(RefCell::new(('Z' as ())));
        let _args_8: Value<()> = Rc::new(RefCell::new(('!' as ())));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>)>(4)
            .call(
                _expected,
                _fmt,
                _args_2.as_pointer(),
                _args_3.as_pointer(),
                _args_4.as_pointer(),
                _args_5.as_pointer(),
                _args_6.as_pointer(),
                _args_7.as_pointer(),
                _args_8.as_pointer(),
            )
    });
    {}
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('0' as ()),
            ('9' as ()),
            ('a' as ()),
            ('z' as ()),
            ('A' as ()),
            ('Z' as ()),
            ('!' as ()),
            0_(),
        ])));
        let data: Value<Ptr<()>> = Rc::new(RefCell::new((buffer.as_pointer() as Ptr<()>)));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_36: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__char_ptrref_::new({ __tmp_36.as_pointer() })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<Ptr<()>>)>(5).call(
                _expected,
                _fmt,
                data.as_pointer(),
            )
        });
    }
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('0' as ()),
            ('9' as ()),
            ('a' as ()),
            ('z' as ()),
            ('A' as ()),
            ('Z' as ()),
            ('!' as ()),
            0_(),
        ])));
        let data: Value<Ptr<()>> = Rc::new(RefCell::new((buffer.as_pointer() as Ptr<()>)));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_37: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__const_char_ptrref_::new({ __tmp_37.as_pointer() })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<Ptr<()>>)>(6).call(
                _expected,
                _fmt,
                data.as_pointer(),
            )
        });
    }
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('a' as ()),
            ('b' as ()),
            ('c' as ()),
            0_(),
            ('d' as ()),
            ('e' as ()),
            ('f' as ()),
            0_(),
        ])));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_38: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__char__ref_arr8arr_::new({ __tmp_38.as_pointer() })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<()>)>(7).call(
                _expected,
                _fmt,
                buffer.as_pointer(),
            )
        });
        let buffer2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('a' as ()),
            ('b' as ()),
            ('c' as ()),
            0_(),
            ('d' as ()),
            ('e' as ()),
            ('f' as ()),
        ])));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_39: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__char__ref_arr7arr_::new({ __tmp_39.as_pointer() })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<()>)>(8).call(
                _expected,
                _fmt,
                buffer2.as_pointer(),
            )
        });
    }
    {
        let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_40: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_ref_::new({ __tmp_40.as_pointer() })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<()>)>(9).call(
                _expected,
                _fmt,
                data.as_pointer(),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let data: Value<()> = Rc::new(RefCell::new(({ (*buffer.borrow())() })));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_41: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_ref_::new({ __tmp_41.as_pointer() })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<()>)>(10).call(
                _expected,
                _fmt,
                data.as_pointer(),
            )
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_42: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__bool__bool_::new({ __tmp_42.as_pointer() })
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(false));
        let _args_3: Value<()> = Rc::new(RefCell::new(true));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>)>(2)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_43: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__signed_char_::new({ __tmp_43.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_44: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__short_::new({ __tmp_44.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(12)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_45: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__int_::new({ __tmp_45.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(13)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_46: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__long_::new({ __tmp_46.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(14)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_47: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__long_long_::new({ __tmp_47.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(15)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_48: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char____int128_::new({ __tmp_48.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(16)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_49: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__unsigned_char_::new({ __tmp_49.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(17)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_50: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__unsigned_short_::new({ __tmp_50.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(18)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_51: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__unsigned_int_::new({ __tmp_51.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(19)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_52: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__unsigned_long_::new({ __tmp_52.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(20)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_53: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__unsigned_long_long_::new({ __tmp_53.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(21)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_54: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__unsigned___int128_::new({ __tmp_54.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(22)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_55: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__float_::new({ __tmp_55.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(23)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_56: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__double_::new({ __tmp_56.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(24)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_57: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__long_double_::new({ __tmp_57.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(25)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_58: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_nullptr_t_::new({ __tmp_58.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(Default::default()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(26)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_59: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__void_ptr_::new({ __tmp_59.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(66_().reinterpret_cast::<()>()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(27)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_60: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__const_void_ptr_::new({ __tmp_60.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(66_().reinterpret_cast::<()>()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(28)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        format_test_handle_4(
            (*check.borrow()).copy_from(),
            (*check_exception.borrow()).copy_from(),
        )
    });
    {}
}
pub fn format_tests_7(check: FnPtr<Generic>, check_exception: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    let check_exception: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check_exception));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_61: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t_::new({ __tmp_61.as_pointer() })
        };
        (*check.borrow())
            .spec::<fn((), ())>(31)
            .call(_expected, _fmt)
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_62: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t_::new({ __tmp_62.as_pointer() })
        };
        (*check.borrow())
            .spec::<fn((), ())>(31)
            .call(_expected, _fmt)
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_63: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t_::new({ __tmp_63.as_pointer() })
        };
        (*check.borrow())
            .spec::<fn((), ())>(31)
            .call(_expected, _fmt)
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_64: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__wchar_t_::new({ __tmp_64.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new((' ' as ())));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(32)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_65: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t_::new({ __tmp_65.as_pointer() })
        };
        (*check.borrow())
            .spec::<fn((), ())>(31)
            .call(_expected, _fmt)
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_66: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__wchar_t_::new({ __tmp_66.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new((' ' as ())));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(32)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_67: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__bool__bool_::new({ __tmp_67.as_pointer() })
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(false));
        let _args_3: Value<()> = Rc::new(RefCell::new(true));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>)>(33)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_68: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__bool__bool_::new({ __tmp_68.as_pointer() })
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(false));
        let _args_3: Value<()> = Rc::new(RefCell::new(true));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>)>(33)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_69: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ :: new ( { __tmp_69.as_pointer() } , )
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(1_()));
        let _args_3: Value<()> = Rc::new(RefCell::new(2_()));
        let _args_4: Value<()> = Rc::new(RefCell::new(3_()));
        let _args_5: Value<()> = Rc::new(RefCell::new(4_()));
        let _args_6: Value<()> = Rc::new(RefCell::new(5_()));
        let _args_7: Value<()> = Rc::new(RefCell::new(6_()));
        let _args_8: Value<()> = Rc::new(RefCell::new(7_()));
        let _args_9: Value<()> = Rc::new(RefCell::new(8_()));
        let _args_10: Value<()> = Rc::new(RefCell::new(9_()));
        let _args_11: Value<()> = Rc::new(RefCell::new(0_()));
        let _args_12: Value<()> = Rc::new(RefCell::new(1_()));
        let _args_13: Value<()> = Rc::new(RefCell::new(2_()));
        let _args_14: Value<()> = Rc::new(RefCell::new(3_()));
        let _args_15: Value<()> = Rc::new(RefCell::new(4_()));
        let _args_16: Value<()> = Rc::new(RefCell::new(5_()));
        let _args_17: Value<()> = Rc::new(RefCell::new(6_()));
        let _args_18: Value<()> = Rc::new(RefCell::new(7_()));
        let _args_19: Value<()> = Rc::new(RefCell::new(8_()));
        let _args_20: Value<()> = Rc::new(RefCell::new(9_()));
        let _args_21: Value<()> = Rc::new(RefCell::new(0_()));
        (*check.borrow())
            .spec::<fn(
                (),
                (),
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
                Ptr<()>,
            )>(34)
            .call(
                _expected,
                _fmt,
                _args_2.as_pointer(),
                _args_3.as_pointer(),
                _args_4.as_pointer(),
                _args_5.as_pointer(),
                _args_6.as_pointer(),
                _args_7.as_pointer(),
                _args_8.as_pointer(),
                _args_9.as_pointer(),
                _args_10.as_pointer(),
                _args_11.as_pointer(),
                _args_12.as_pointer(),
                _args_13.as_pointer(),
                _args_14.as_pointer(),
                _args_15.as_pointer(),
                _args_16.as_pointer(),
                _args_17.as_pointer(),
                _args_18.as_pointer(),
                _args_19.as_pointer(),
                _args_20.as_pointer(),
                _args_21.as_pointer(),
            )
    });
    {}
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(4)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(4)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(4)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(4)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        (*check_exception.borrow())
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    ({
        let _arg2: Value<()> = Rc::new(RefCell::new(42_()));
        (*check_exception.borrow())
            .spec::<fn((), (), Ptr<()>)>(4)
            .call(unimplemented!(), unimplemented!(), _arg2.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_70: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t_ :: new ( { __tmp_70.as_pointer() } , )
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(('0' as ())));
        let _args_3: Value<()> = Rc::new(RefCell::new(('9' as ())));
        let _args_4: Value<()> = Rc::new(RefCell::new(('a' as ())));
        let _args_5: Value<()> = Rc::new(RefCell::new(('z' as ())));
        let _args_6: Value<()> = Rc::new(RefCell::new(('A' as ())));
        let _args_7: Value<()> = Rc::new(RefCell::new(('Z' as ())));
        let _args_8: Value<()> = Rc::new(RefCell::new(('!' as ())));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>)>(35)
            .call(
                _expected,
                _fmt,
                _args_2.as_pointer(),
                _args_3.as_pointer(),
                _args_4.as_pointer(),
                _args_5.as_pointer(),
                _args_6.as_pointer(),
                _args_7.as_pointer(),
                _args_8.as_pointer(),
            )
    });
    {}
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('0' as ()),
            ('9' as ()),
            ('a' as ()),
            ('z' as ()),
            ('A' as ()),
            ('Z' as ()),
            ('!' as ()),
            0_(),
        ])));
        let data: Value<Ptr<()>> = Rc::new(RefCell::new((buffer.as_pointer() as Ptr<()>)));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_71: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_wchar_t__wchar_t_ptrref_::new({ __tmp_71.as_pointer() })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<Ptr<()>>)>(36).call(
                _expected,
                _fmt,
                data.as_pointer(),
            )
        });
    }
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('0' as ()),
            ('9' as ()),
            ('a' as ()),
            ('z' as ()),
            ('A' as ()),
            ('Z' as ()),
            ('!' as ()),
            0_(),
        ])));
        let data: Value<Ptr<()>> = Rc::new(RefCell::new((buffer.as_pointer() as Ptr<()>)));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_72: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_wchar_t__const_wchar_t_ptrref_::new({
                    __tmp_72.as_pointer()
                })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<Ptr<()>>)>(37).call(
                _expected,
                _fmt,
                data.as_pointer(),
            )
        });
    }
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('a' as ()),
            ('b' as ()),
            ('c' as ()),
            0_(),
            ('d' as ()),
            ('e' as ()),
            ('f' as ()),
            0_(),
        ])));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_73: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_wchar_t__wchar_t__ref_arr8arr_::new({
                    __tmp_73.as_pointer()
                })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<()>)>(38).call(
                _expected,
                _fmt,
                buffer.as_pointer(),
            )
        });
        let buffer2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('a' as ()),
            ('b' as ()),
            ('c' as ()),
            0_(),
            ('d' as ()),
            ('e' as ()),
            ('f' as ()),
        ])));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_74: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_wchar_t__wchar_t__ref_arr7arr_::new({
                    __tmp_74.as_pointer()
                })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<()>)>(39).call(
                _expected,
                _fmt,
                buffer2.as_pointer(),
            )
        });
    }
    {
        let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_75: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_wchar_t__std_wstring_ref_::new({ __tmp_75.as_pointer() })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<()>)>(40).call(
                _expected,
                _fmt,
                data.as_pointer(),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let data: Value<()> = Rc::new(RefCell::new(({ (*buffer.borrow())() })));
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_76: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_wchar_t__std_wstring_view_ref_::new({
                    __tmp_76.as_pointer()
                })
            };
            (*check.borrow()).spec::<fn((), (), Ptr<()>)>(41).call(
                _expected,
                _fmt,
                data.as_pointer(),
            )
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_77: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__bool__bool_::new({ __tmp_77.as_pointer() })
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(false));
        let _args_3: Value<()> = Rc::new(RefCell::new(true));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>)>(33)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_78: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__signed_char_::new({ __tmp_78.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(42)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_79: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__short_::new({ __tmp_79.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(43)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_80: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__int_::new({ __tmp_80.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(44)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_81: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__long_::new({ __tmp_81.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(45)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_82: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__long_long_::new({ __tmp_82.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(46)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_83: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t____int128_::new({ __tmp_83.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(47)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_84: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__unsigned_char_::new({ __tmp_84.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(48)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_85: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__unsigned_short_::new({ __tmp_85.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(49)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_86: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__unsigned_int_::new({ __tmp_86.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(50)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_87: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__unsigned_long_::new({ __tmp_87.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(51)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_88: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__unsigned_long_long_::new({ __tmp_88.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(52)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_89: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__unsigned___int128_::new({ __tmp_89.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(53)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_90: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__float_::new({ __tmp_90.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(54)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_91: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__double_::new({ __tmp_91.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(55)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_92: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__long_double_::new({ __tmp_92.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(42_()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(56)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_93: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_nullptr_t_::new({ __tmp_93.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(Default::default()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(57)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_94: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__void_ptr_::new({ __tmp_94.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(66_().reinterpret_cast::<()>()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(58)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_95: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__const_void_ptr_::new({ __tmp_95.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(66_().reinterpret_cast::<()>()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(59)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        format_test_handle_5(
            (*check.borrow()).copy_from(),
            (*check_exception.borrow()).copy_from(),
        )
    });
    {}
}
pub fn format_tests_char_to_wchar_t_8(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_96: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__char__char__char__char__char_::new({
                __tmp_96.as_pointer()
            })
        };
        let _args_2: Value<()> = Rc::new(RefCell::new(('0' as ())));
        let _args_3: Value<()> = Rc::new(RefCell::new(('9' as ())));
        let _args_4: Value<()> = Rc::new(RefCell::new(('a' as ())));
        let _args_5: Value<()> = Rc::new(RefCell::new(('z' as ())));
        let _args_6: Value<()> = Rc::new(RefCell::new(('A' as ())));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>, Ptr<()>)>(30)
            .call(
                _expected,
                _fmt,
                _args_2.as_pointer(),
                _args_3.as_pointer(),
                _args_4.as_pointer(),
                _args_5.as_pointer(),
                _args_6.as_pointer(),
            )
    });
}
thread_local!(
    pub static test_9: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: ()| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args_2: Ptr<()>, args_3: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (),
         fmt: (),
         args_2: Ptr<()>,
         args_3: Ptr<()>,
         args_4: Ptr<()>,
         args_5: Ptr<()>,
         args_6: Ptr<()>,
         args_7: Ptr<()>,
         args_8: Ptr<()>,
         args_9: Ptr<()>,
         args_10: Ptr<()>,
         args_11: Ptr<()>,
         args_12: Ptr<()>,
         args_13: Ptr<()>,
         args_14: Ptr<()>,
         args_15: Ptr<()>,
         args_16: Ptr<()>,
         args_17: Ptr<()>,
         args_18: Ptr<()>,
         args_19: Ptr<()>,
         args_20: Ptr<()>,
         args_21: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (),
         fmt: (),
         args_2: Ptr<()>,
         args_3: Ptr<()>,
         args_4: Ptr<()>,
         args_5: Ptr<()>,
         args_6: Ptr<()>,
         args_7: Ptr<()>,
         args_8: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<Ptr<()>>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<Ptr<()>>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<status>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () =
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { 10_() },
                        { (' ' as ()) },
                        None,
                    );
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (),
         fmt: (),
         args_2: Ptr<()>,
         args_3: Ptr<()>,
         args_4: Ptr<()>,
         args_5: Ptr<()>,
         args_6: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: ()| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args_2: Ptr<()>, args_3: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (),
         fmt: (),
         args_2: Ptr<()>,
         args_3: Ptr<()>,
         args_4: Ptr<()>,
         args_5: Ptr<()>,
         args_6: Ptr<()>,
         args_7: Ptr<()>,
         args_8: Ptr<()>,
         args_9: Ptr<()>,
         args_10: Ptr<()>,
         args_11: Ptr<()>,
         args_12: Ptr<()>,
         args_13: Ptr<()>,
         args_14: Ptr<()>,
         args_15: Ptr<()>,
         args_16: Ptr<()>,
         args_17: Ptr<()>,
         args_18: Ptr<()>,
         args_19: Ptr<()>,
         args_20: Ptr<()>,
         args_21: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (),
         fmt: (),
         args_2: Ptr<()>,
         args_3: Ptr<()>,
         args_4: Ptr<()>,
         args_5: Ptr<()>,
         args_6: Ptr<()>,
         args_7: Ptr<()>,
         args_8: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<Ptr<()>>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<Ptr<()>>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<status>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out : ()  = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_()  } , { ( ' ' as ()  )  } , None , )   ;
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                let mut size: () = ((unimplemented!().read()) as ());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    )));
);
thread_local!(
    pub static test_exception_10: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |_a0: (), _a1: ()| {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        },
        |_a0: (), _a1: (), _a2: Ptr<()>| {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        },
        |_a0: (), _a1: (), _a2: Ptr<status>| {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        },
        |_a0: (), _a1: ()| {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        },
        |_a0: (), _a1: (), _a2: Ptr<()>| {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        },
        |_a0: (), _a1: (), _a2: Ptr<status>| {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        }
    )));
);

pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({
        format_tests_6(
            (*test_9.with(Value::clone).borrow()).copy_from(),
            (*test_exception_10.with(Value::clone).borrow()).copy_from(),
        )
    });
    ({ format_tests_char_to_wchar_t_8((*test_9.with(Value::clone).borrow()).copy_from()) });
    ({
        format_tests_7(
            (*test_9.with(Value::clone).borrow()).copy_from(),
            (*test_exception_10.with(Value::clone).borrow()).copy_from(),
        )
    });
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = test_9.with(|_| ());
    let _ = test_exception_10.with(|_| ());
}
