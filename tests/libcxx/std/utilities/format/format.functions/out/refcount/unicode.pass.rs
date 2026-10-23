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
pub fn check_0(expected: (), fmt: (), args: Ptr<()>) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    unimplemented!();
}
pub fn check_1(expected: (), fmt: (), args: Ptr<()>) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {}
    unimplemented!();
}
pub fn test_single_code_point_fill_2() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_0.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_1.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_2.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_3.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_4.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_5.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_6.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_7: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_7.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_8.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_9.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_10.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_11.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_12: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_12.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_13: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_13.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_14: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_14.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_15: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_15.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_16: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_16.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_17: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_17.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_18: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_18.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_19: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_19.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_20: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_20.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_21: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_21.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_22: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_22.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_23: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_23.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_24: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_24.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_25: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_25.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_26: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_26.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_27: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_27.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_28: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_28.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_29: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_29.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_30: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_30.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_31: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_31.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_32: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_32.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_33: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_33.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_34: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_34.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_35: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_35.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_single_code_point_fill_3() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_36: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_36.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_37: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_37.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_38: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_38.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_39: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_39.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_40: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_40.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_41: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_41.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_42: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_42.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_43: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_43.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_44: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_44.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_45: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_45.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_46: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_46.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_47: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_47.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_48: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_48.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_49: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_49.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_50: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_50.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_51: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_51.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_52: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_52.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_53: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_53.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_54: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_54.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_55: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_55.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_56: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_56.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_57: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_57.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_58: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_58.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_59: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_59.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_60: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_60.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_61: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_61.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_62: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_62.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_63: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_63.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_64: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_64.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_65: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_65.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_66: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_66.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_67: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_67.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_68: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_68.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_69: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_69.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_70: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_70.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_71: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_71.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_single_code_point_truncate_4() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_72: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_72.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_73: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_73.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_74: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_74.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_75: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_75.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_76: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_76.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_77: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_77.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_78: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_78.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_79: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_79.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_80: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_80.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_81: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_81.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_82: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_82.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_83: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_83.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_84: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_84.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_85: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_85.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_86: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_86.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_87: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_87.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_88: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_88.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_89: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_89.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_90: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_90.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_91: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_91.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_92: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_92.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_93: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_93.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_94: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_94.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_95: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_95.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_96: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_96.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_97: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_97.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_98: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_98.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_99: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_99.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_100: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_100.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_101: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_101.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_102: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_102.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_103: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_103.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_104: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_104.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_105: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_105.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_106: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_106.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_107: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_107.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_single_code_point_truncate_5() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_108: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_108.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_109: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_109.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_110: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_110.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_111: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_111.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_112: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_112.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_113: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_113.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_114: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_114.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_115: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_115.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_116: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_116.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_117: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_117.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_118: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_118.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_119: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_119.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_120: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_120.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_121: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_121.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_122: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_122.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_123: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_123.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_124: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_124.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_125: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_125.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_126: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_126.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_127: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_127.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_128: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_128.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_129: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_129.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_130: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_130.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_131: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_131.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_132: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_132.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_133: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_133.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_134: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_134.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_135: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_135.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_136: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_136.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_137: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_137.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_138: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_138.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_139: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_139.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_140: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_140.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_141: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_141.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_142: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_142.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_143: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_143.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_P1868_6() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_144: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_144.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_145: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_145.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_146: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_146.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_147: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_147.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_148: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_148.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_149: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_149.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_150: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_150.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_151: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_151.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_152: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_152.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_153: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_153.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_154: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_154.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_155: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_155.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_156: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_156.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_157: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_157.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_158: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_158.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_159: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_159.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_160: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_160.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_161: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_161.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_162: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_162.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_163: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_163.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_164: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_164.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_165: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_165.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_166: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_166.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_167: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_167.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_168: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_168.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_169: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_169.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_170: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_170.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_171: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_171.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_172: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_172.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_173: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_173.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_174: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_174.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_175: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_175.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_176: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_176.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_177: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_177.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_178: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_178.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_179: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_char__std_string_view_::new({ __tmp_179.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_0(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_P1868_7() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_180: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_180.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_181: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_181.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_182: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_182.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_183: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_183.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_184: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_184.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_185: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_185.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_186: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_186.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_187: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_187.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_188: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_188.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_189: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_189.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_190: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_190.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_191: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_191.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_192: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_192.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_193: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_193.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_194: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_194.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_195: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_195.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_196: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_196.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_197: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_197.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_198: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_198.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_199: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_199.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_200: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_200.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_201: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_201.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_202: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_202.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_203: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_203.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_204: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_204.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_205: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_205.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_206: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_206.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_207: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_207.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_208: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_208.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_209: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_209.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_210: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_210.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_211: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_211.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_212: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_212.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_213: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_213.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_214: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_214.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = {
            let __tmp_215: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std_basic_format_string_wchar_t__std_wstring_view_::new({ __tmp_215.as_pointer() })
        };
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_1(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_malformed_code_point_8() {
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_216: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_216.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_217: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_217.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_218: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_218.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_219: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_219.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_220: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_220.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_221: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_221.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_222: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_222.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_223: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_223.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_224: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_224.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_225: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_225.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_226: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_226.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_227: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_227.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_228: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_228.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_229: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_229.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_230: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_230.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_231: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_231.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_232: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_232.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_233: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_233.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_234: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_234.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_235: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_235.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_236: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_236.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_237: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_237.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_238: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_238.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_239: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_239.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_240: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_240.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_241: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_241.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_242: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_242.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_243: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_243.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_244: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_244.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_245: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_245.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_246: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_246.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_247: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_247.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_248: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_248.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_249: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_249.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_250: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_250.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_251: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_251.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_252: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_252.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_253: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_253.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_254: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_254.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_255: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_255.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_256: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_256.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = {
                let __tmp_257: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                std_basic_format_string_char__std_string_view_::new({ __tmp_257.as_pointer() })
            };
            let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_0(_expected, _fmt, _args.as_pointer())
        });
    }
}
pub fn test_malformed_code_point_9() {
    { {} }
}
pub fn test_10() {
    ({ test_single_code_point_fill_2() });
    ({ test_single_code_point_truncate_4() });
    ({ test_P1868_6() });
    ({ test_malformed_code_point_8() });
}
pub fn test_11() {
    ({ test_single_code_point_fill_3() });
    ({ test_single_code_point_truncate_5() });
    ({ test_P1868_7() });
    ({ test_malformed_code_point_9() });
}
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
    ({ test_10() });
    ({ test_11() });
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
pub fn __cpp2rust_init_globals() {}
