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
pub unsafe fn check_0(
    mut expected: (),
    mut fmt: std_basic_format_string_char__std_string_view_,
    args: *mut (),
) {
    let mut out: () = unimplemented!();
    {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    unimplemented!();
}
pub unsafe fn check_1(
    mut expected: (),
    mut fmt: std_basic_format_string_wchar_t__std_wstring_view_,
    args: *mut (),
) {
    let mut out: () = unimplemented!();
    {}
    unimplemented!();
}
pub unsafe fn test_single_code_point_fill_2() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_0: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_0 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_1: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_1 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_2: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_2 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_3: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_3 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_4: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_4 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_5: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_5 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_6: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_6 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_7: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_7 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_8: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_8 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_9: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_9 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_10: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_10 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_11: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_11 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_12: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_12 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_13: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_13 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_14: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_14 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_15: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_15 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_16: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_16 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_17: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_17 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_18: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_18 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_19: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_19 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_20: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_20 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_21: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_21 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_22: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_22 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_23: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_23 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_24: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_24 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_25: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_25 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_26: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_26 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_27: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_27 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_28: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_28 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_29: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_29 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_30: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_30 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_31: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_31 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_32: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_32 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_33: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_33 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_34: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_34 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_35: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_35 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_single_code_point_fill_3() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_36: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_36 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_37: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_37 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_38: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_38 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_39: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_39 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_40: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_40 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_41: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_41 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_42: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_42 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_43: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_43 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_44: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_44 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_45: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_45 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_46: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_46 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_47: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_47 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_48: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_48 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_49: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_49 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_50: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_50 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_51: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_51 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_52: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_52 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_53: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_53 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_54: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_54 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_55: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_55 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_56: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_56 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_57: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_57 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_58: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_58 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_59: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_59 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_60: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_60 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_61: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_61 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_62: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_62 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_63: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_63 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_64: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_64 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_65: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_65 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_66: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_66 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_67: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_67 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_68: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_68 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_69: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_69 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_70: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_70 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_71: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_71 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_single_code_point_truncate_4() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_72: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_72 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_73: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_73 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_74: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_74 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_75: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_75 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_76: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_76 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_77: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_77 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_78: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_78 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_79: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_79 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_80: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_80 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_81: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_81 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_82: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_82 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_83: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_83 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_84: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_84 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_85: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_85 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_86: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_86 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_87: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_87 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_88: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_88 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_89: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_89 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_90: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_90 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_91: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_91 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_92: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_92 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_93: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_93 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_94: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_94 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_95: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_95 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_96: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_96 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_97: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_97 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_98: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_98 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_99: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_99 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_100: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_100 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_101: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_101 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_102: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_102 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_103: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_103 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_104: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_104 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_105: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_105 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_106: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_106 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_107: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_107 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_single_code_point_truncate_5() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_108: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_108 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_109: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_109 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_110: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_110 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_111: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_111 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_112: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_112 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_113: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_113 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_114: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_114 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_115: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_115 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_116: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_116 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_117: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_117 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_118: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_118 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_119: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_119 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_120: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_120 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_121: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_121 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_122: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_122 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_123: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_123 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_124: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_124 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_125: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_125 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_126: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_126 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_127: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_127 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_128: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_128 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_129: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_129 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_130: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_130 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_131: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_131 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_132: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_132 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_133: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_133 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_134: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_134 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_135: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_135 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_136: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_136 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_137: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_137 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_138: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_138 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_139: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_139 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_140: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_140 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_141: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_141 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_142: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_142 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_143: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_143 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_P1868_6() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_144: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_144 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_145: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_145 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_146: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_146 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_147: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_147 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_148: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_148 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_149: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_149 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_150: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_150 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_151: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_151 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_152: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_152 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_153: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_153 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_154: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_154 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_155: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_155 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_156: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_156 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_157: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_157 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_158: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_158 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_159: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_159 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_160: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_160 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_161: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_161 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_162: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_162 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_163: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_163 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_164: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_164 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_165: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_165 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_166: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_166 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_167: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_167 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_168: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_168 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_169: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_169 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_170: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_170 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_171: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_171 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_172: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_172 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_173: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_173 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_174: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_174 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_175: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_175 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_176: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_176 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_177: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_177 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_178: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_178 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_179: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_179 })
        };
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_P1868_7() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_180: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_180 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_181: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_181 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_182: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_182 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_183: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_183 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_184: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_184 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_185: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_185 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_186: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_186 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_187: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_187 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_188: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_188 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_189: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_189 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_190: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_190 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_191: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_191 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_192: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_192 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_193: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_193 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_194: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_194 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_195: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_195 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_196: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_196 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_197: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_197 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_198: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_198 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_199: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_199 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_200: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_200 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_201: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_201 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_202: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_202 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_203: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_203 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_204: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_204 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_205: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_205 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_206: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_206 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_207: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_207 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_208: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_208 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_209: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_209 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_210: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_210 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_211: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_211 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_212: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_212 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_213: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_213 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_214: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_214 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_215: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_215 })
        };
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_malformed_code_point_8() {
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_216: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_216 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_217: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_217 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_218: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_218 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_219: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_219 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_220: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_220 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_221: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_221 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_222: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_222 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_223: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_223 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_224: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_224 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_225: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_225 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_226: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_226 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_227: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_227 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_228: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_228 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_229: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_229 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_230: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_230 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_231: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_231 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_232: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_232 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_233: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_233 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_234: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_234 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_235: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_235 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_236: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_236 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_237: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_237 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_238: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_238 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_239: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_239 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_240: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_240 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_241: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_241 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_242: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_242 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_243: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_243 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_244: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_244 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_245: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_245 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_246: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_246 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_247: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_247 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_248: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_248 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_249: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_249 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_250: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_250 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_251: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_251 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_252: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_252 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_253: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_253 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_254: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_254 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_255: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_255 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_256: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_256 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = {
                let mut __tmp_257: () = unimplemented!();
                std_basic_format_string_char__std_string_view_::new({ &mut __tmp_257 })
            };
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
    }
}
pub unsafe fn test_malformed_code_point_9() {
    { {} }
}
pub unsafe fn test_10() {
    (unsafe { test_single_code_point_fill_2() });
    (unsafe { test_single_code_point_truncate_4() });
    (unsafe { test_P1868_6() });
    (unsafe { test_malformed_code_point_8() });
}
pub unsafe fn test_11() {
    (unsafe { test_single_code_point_fill_3() });
    (unsafe { test_single_code_point_truncate_5() });
    (unsafe { test_P1868_7() });
    (unsafe { test_malformed_code_point_9() });
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
    (unsafe { test_10() });
    (unsafe { test_11() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
