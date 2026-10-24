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
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_single_code_point_fill_3() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_single_code_point_truncate_4() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_single_code_point_truncate_5() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_P1868_6() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_0(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_P1868_7() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = unimplemented!();
        let mut _args: () = unimplemented!();
        check_1(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_malformed_code_point_8() {
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
            let mut _args: () = unimplemented!();
            check_0(_expected, _fmt, &mut _args)
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ = unimplemented!();
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
