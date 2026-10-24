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
pub unsafe fn get_temp_file_name_19() -> () {
    let mut Name: () = unimplemented!();
    let mut FD: () = (unsafe { mkstemp_20((&mut (*unimplemented!()) as *mut ())) });
    if ((FD) == (-1_())) {
        (unsafe { perror_21(c"mkstemp".as_ptr()) });
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn glibc_version_less_than_22(mut version: *const ()) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = (unsafe {
        confstr_23(
            (anon_24__CS_GNU_LIBC_VERSION as ()),
            std::ptr::null_mut(),
            0_(),
        )
    });
    let mut current_version: *mut () =
        Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    (unsafe { confstr_23((anon_24__CS_GNU_LIBC_VERSION as ()), current_version, n) });
    let mut result: () =
        ((unsafe { strverscmp_25((current_version).cast_const(), unimplemented!()) }) < (0_()));
    {
        let __p = current_version;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    return result;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct test_range_format_string_std_string_ {
    str_: (),
}
impl test_range_format_string_std_string_ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
impl Default for test_range_format_string_std_string_ {
    fn default() -> Self {
        unsafe {
            test_range_format_string_std_string_ {
                str_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct test_range_format_string_std_string_view_ {
    str_: (),
}
impl test_range_format_string_std_string_view_ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct test_range_format_string_std_list_char__std_allocator_char___ {
    str_: (),
}
impl test_range_format_string_std_list_char__std_allocator_char___ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct test_range_format_string_std_wstring_ {
    str_: (),
}
impl test_range_format_string_std_wstring_ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
impl Default for test_range_format_string_std_wstring_ {
    fn default() -> Self {
        unsafe {
            test_range_format_string_std_wstring_ {
                str_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct test_range_format_string_std_wstring_view_ {
    str_: (),
}
impl test_range_format_string_std_wstring_view_ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ {
    str_: (),
}
impl test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct test_range_format_debug_string_std_string_ {
    str_: (),
}
impl test_range_format_debug_string_std_string_ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
impl Default for test_range_format_debug_string_std_string_ {
    fn default() -> Self {
        unsafe {
            test_range_format_debug_string_std_string_ {
                str_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct test_range_format_debug_string_std_string_view_ {
    str_: (),
}
impl test_range_format_debug_string_std_string_view_ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct test_range_format_debug_string_std_list_char__std_allocator_char___ {
    str_: (),
}
impl test_range_format_debug_string_std_list_char__std_allocator_char___ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct test_range_format_debug_string_std_wstring_ {
    str_: (),
}
impl test_range_format_debug_string_std_wstring_ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
impl Default for test_range_format_debug_string_std_wstring_ {
    fn default() -> Self {
        unsafe {
            test_range_format_debug_string_std_wstring_ {
                str_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct test_range_format_debug_string_std_wstring_view_ {
    str_: (),
}
impl test_range_format_debug_string_std_wstring_view_ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ {
    str_: (),
}
impl test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ {
    pub unsafe fn new(mut str: ()) -> Self {
        let mut this = Self {
            str_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&self) -> () {
        return unimplemented!();
    }
}
pub unsafe fn test_string_26(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_string_std_string_,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_, *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_, *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_, *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_, *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_ = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_, *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_, *mut (), *mut ())>(2)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_0: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_string_std_string_ = input;
            check_exception
                .spec::<fn((), (), *mut test_range_format_string_std_string_)>(0)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_string_27(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_string_std_string_view_,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_view_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_, *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_view_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_, *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_view_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_, *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_view_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_, *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(1)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(1)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(1)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(1)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(1)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_view_ = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_, *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_string_view_ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_, *mut (), *mut ())>(5)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(1)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(3)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_1: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_1;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_string_std_string_view_ = input;
            check_exception
                .spec::<fn((), (), *mut test_range_format_string_std_string_view_)>(1)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_string_28(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_string_std_list_char__std_allocator_char___,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_char__std_allocator_char___ ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_char__std_allocator_char___ ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_char__std_allocator_char___ ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_char__std_allocator_char___ ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                2,
            )
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                2,
            )
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                2,
            )
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                2,
            )
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                2,
            )
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        let mut _args_3: () = 3_();
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_char__std_allocator_char___ ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_string_std_list_char__std_allocator_char___,
                *mut (),
                *mut (),
            )>(8)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                2,
            )
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_list_char__std_allocator_char___)>(
                6,
            )
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_2: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_2;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_string_std_list_char__std_allocator_char___ = input;
            check_exception  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_char__std_allocator_char___ ,)>(2) .call ( unimplemented!() , _fmt , _args , )
        });
    }
}
pub unsafe fn test_string_29(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_string_std_wstring_,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_, *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_, *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_, *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_, *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(22)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(22)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(22)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(22)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(22)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_ = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_, *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_, *mut (), *mut ())>(44)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(22)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(42)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_3: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_3;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_string_std_wstring_ = input;
            check_exception
                .spec::<fn((), (), *mut test_range_format_string_std_wstring_)>(22)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_string_30(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_string_std_wstring_view_,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_view_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_, *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_view_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_, *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_view_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_, *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_view_ = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_, *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_view_ = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_, *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_wstring_view_ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_, *mut (), *mut ())>(
                47,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(45)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_4: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_4;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_string_std_wstring_view_ = input;
            check_exception
                .spec::<fn((), (), *mut test_range_format_string_std_wstring_view_)>(23)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_string_31(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(24) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(24) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(24) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(24) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(24) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
                *mut (),
            )>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(24) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(48) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_5: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_5;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ =
                input;
            check_exception  .spec::<fn(() ,() ,*mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ ,)>(24) .call ( unimplemented!() , _fmt , _args , )
        });
    }
}
pub unsafe fn test_string_32(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: test_range_format_string_std_string_ =
            test_range_format_string_std_string_::new({ unimplemented!() });
        test_string_26(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: test_range_format_string_std_string_view_ =
            test_range_format_string_std_string_view_::new({ (unsafe { input() }) });
        test_string_27(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: test_range_format_string_std_list_char__std_allocator_char___ =
            test_range_format_string_std_list_char__std_allocator_char___::new({
                unimplemented!()
            });
        test_string_28(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_string_33(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: test_range_format_string_std_wstring_ =
            test_range_format_string_std_wstring_::new({ unimplemented!() });
        test_string_29(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: test_range_format_string_std_wstring_view_ =
            test_range_format_string_std_wstring_view_::new({ (unsafe { input() }) });
        test_string_30(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ =
            test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___::new({
                unimplemented!()
            });
        test_string_31(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_range_string_34(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_6: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_6;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(3)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_7: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_7;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(3)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_string_35(
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
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
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
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_8: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_8;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(5)
                .call(unimplemented!(), _fmt, _args)
        });
    }
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
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
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
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_9: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_9;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(5)
                .call(unimplemented!(), _fmt, _args)
        });
    }
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
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(14)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
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
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(14)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_string_36(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_10: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_10;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(7)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_11: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_11;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(7)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(17)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(17)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_string_37(
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
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
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
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_12: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_12;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(9)
                .call(unimplemented!(), _fmt, _args)
        });
    }
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
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
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
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_13: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_13;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(9)
                .call(unimplemented!(), _fmt, _args)
        });
    }
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
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(20)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
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
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(20)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_string_38(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_14: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_14;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(25)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_15: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_15;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(25)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(53)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(52)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(53)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(26)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_string_39(
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
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
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
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_16: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_16;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(27)
                .call(unimplemented!(), _fmt, _args)
        });
    }
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
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
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
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_17: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_17;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(27)
                .call(unimplemented!(), _fmt, _args)
        });
    }
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
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(56)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
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
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(56)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_string_40(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_18: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_18;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(29)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_19: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_19;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(29)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(30)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_string_41(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 19_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_20: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_20;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(31)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_21: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_21;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(31)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(62)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(62)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(32)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_string_42(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_string_34(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_string_35(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_string_36(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_string_37(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_range_string_43(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_string_38(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_string_39(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_string_40(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_string_41(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_debug_string_44(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_debug_string_std_string_,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_, *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_, *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_, *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_, *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(11)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(11)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(11)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(11)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(11)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_ = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_, *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_, *mut (), *mut ())>(
                23,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(11)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(21)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_22: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_22;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_debug_string_std_string_ = input;
            check_exception
                .spec::<fn((), (), *mut test_range_format_debug_string_std_string_)>(11)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_debug_string_45(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_debug_string_std_string_view_,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_view_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_, *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_view_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_, *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_view_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_, *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_view_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_, *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(12)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(12)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(12)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(12)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(12)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_view_ = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_, *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_string_view_ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_string_view_ ,*mut () ,*mut () ,)>(26) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(12)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_string_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(24)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_23: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_23;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_debug_string_std_string_view_ = input;
            check_exception
                .spec::<fn((), (), *mut test_range_format_debug_string_std_string_view_)>(12)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_debug_string_46(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_debug_string_std_list_char__std_allocator_char___,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ =
            input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_char__std_allocator_char___,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ =
            input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_char__std_allocator_char___,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ =
            input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_char__std_allocator_char___,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ =
            input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_char__std_allocator_char___,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(13) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(13) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(13) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(13) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(13) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ =
            input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_char__std_allocator_char___,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ =
            input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_char__std_allocator_char___,
                *mut (),
                *mut (),
            )>(29)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check_exception  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(13) .call ( unimplemented!() , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ = input;
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(27) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_24: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_24;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_debug_string_std_list_char__std_allocator_char___ =
                input;
            check_exception  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_list_char__std_allocator_char___ ,)>(13) .call ( unimplemented!() , _fmt , _args , )
        });
    }
}
pub unsafe fn test_debug_string_47(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_debug_string_std_wstring_,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_, *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_, *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_, *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_, *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_ = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_, *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_, *mut (), *mut ())>(
                65,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(63)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_25: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_25;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_debug_string_std_wstring_ = input;
            check_exception
                .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_)>(33)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_debug_string_48(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_debug_string_std_wstring_view_,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_view_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_, *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_view_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_, *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_view_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_, *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_view_ = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_, *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(34)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(34)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(34)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(34)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(34)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_view_ = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_, *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_wstring_view_ = input;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check  .spec::<fn(() ,() ,*mut test_range_format_debug_string_std_wstring_view_ ,*mut () ,*mut () ,)>(68) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check_exception
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(34)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
        check
            .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(66)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_26: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_26;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_debug_string_std_wstring_view_ = input;
            check_exception
                .spec::<fn((), (), *mut test_range_format_debug_string_std_wstring_view_)>(34)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_debug_string_49(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___  = input ;
        let mut _args_3: () = 12_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___  = input ;
        let mut _args_3: () = 12_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___  = input ;
        let mut _args_3: () = 12_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___  = input ;
        let mut _args_3: () = 12_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check_exception
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(35)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check_exception
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(35)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check_exception
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(35)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check_exception
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(35)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check_exception
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(35)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___  = input ;
        let mut _args_3: () = 3_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
            )>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___  = input ;
        let mut _args_3: () = 5_();
        let mut _args_4: () = 3_();
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
                *mut (),
                *mut (),
            )>(71)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check_exception
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(35)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            input;
        check
            .spec::<fn(
                (),
                (),
                *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
            )>(69)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_27: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_27;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___  = input ;
            check_exception
                .spec::<fn(
                    (),
                    (),
                    *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___,
                )>(35)
                .call(unimplemented!(), _fmt, _args)
        });
    }
}
pub unsafe fn test_debug_string_50(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: test_range_format_debug_string_std_string_ =
            test_range_format_debug_string_std_string_::new({ unimplemented!() });
        test_debug_string_44(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: test_range_format_debug_string_std_string_view_ =
            test_range_format_debug_string_std_string_view_::new({ (unsafe { input() }) });
        test_debug_string_45(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: test_range_format_debug_string_std_list_char__std_allocator_char___ =
            test_range_format_debug_string_std_list_char__std_allocator_char___::new({
                unimplemented!()
            });
        test_debug_string_46(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_debug_string_51(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: test_range_format_debug_string_std_wstring_ =
            test_range_format_debug_string_std_wstring_::new({ unimplemented!() });
        test_debug_string_47(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: test_range_format_debug_string_std_wstring_view_ =
            test_range_format_debug_string_std_wstring_view_::new({ (unsafe { input() }) });
        test_debug_string_48(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ =
            test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___::new({
                unimplemented!()
            });
        test_debug_string_49(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_range_debug_string_52(
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
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
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
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_28: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_28;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(14)
                .call(unimplemented!(), _fmt, _args)
        });
    }
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
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
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
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_29: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_29;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(14)
                .call(unimplemented!(), _fmt, _args)
        });
    }
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
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(32)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
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
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(32)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_debug_string_53(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_30: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_30;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(16)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_31: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_31;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(16)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(35)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(35)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_debug_string_54(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_32: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_32;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(18)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_33: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_33;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(18)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(38)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(38)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_debug_string_55(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_34: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_34;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(20)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_35: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_35;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(20)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(41)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(41)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_debug_string_56(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_36: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_36;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(36)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_37: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_37;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(36)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(74)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(74)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_debug_string_57(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_38: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_38;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(38)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_39: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_39;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(38)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(77)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(77)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_debug_string_58(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_40: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_40;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(40)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_41: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_41;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(40)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(80)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(78)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(79)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(80)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(41)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_debug_string_59(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_42: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_42;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(42)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_43: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_43;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(42)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(83)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(83)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_range_debug_string_60(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_debug_string_52(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_debug_string_53(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_debug_string_54(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_debug_string_55(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_range_debug_string_61(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_debug_string_56(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_debug_string_57(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_debug_string_58(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_range_debug_string_59(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn format_tests_62(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe { test_string_32(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_range_string_42(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_debug_string_50(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_range_debug_string_60(check.copy_from(), check_exception.copy_from()) });
}
pub unsafe fn format_tests_63(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe { test_string_33(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_range_string_43(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_debug_string_51(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_range_debug_string_61(check.copy_from(), check_exception.copy_from()) });
}
pub unsafe fn test_eprintf_64(
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
pub unsafe fn test_eprintf_65(
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
pub unsafe fn test_eprintf_66(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_67(
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
        test_eprintf_64(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_68(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_65(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_69(
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
        test_log_68(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_70(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_71(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_72(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_73(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_74(mut value: ()) -> () {
    return (unsafe { test_is_code_point_73(value) })
        && (!(unsafe { test_is_surrogate_72(value) }));
}
pub static mut test_replacement_character_75: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn test_encode_76(out_it: *mut (), mut value: ()) {
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
pub unsafe fn test_encode_77(out_it: *mut (), mut value: ()) {
    if (((value) & (4294901760_())) == (0_())) {
        (unsafe {
            let _out_it: *mut () = out_it;
            let _value: () = value;
            test_encode_76(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_transcode_78(mut first: (), mut last: (), mut out_it: ()) -> () {
    'loop_: while !(unimplemented!()) {
        let mut value: () = (*unimplemented!());
        if (unsafe { test_is_code_point_73(value) }) {
            (unsafe { test_encode_77(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_76(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_75)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn test_transcode_79(mut first: *const (), mut last: *const (), mut out_it: ()) -> () {
    'loop_: while ((first) != (last)) {
        let mut value: () = (*first.postfix_inc());
        if (unsafe { test_is_code_point_73(value) }) {
            (unsafe { test_encode_77(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_76(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_75)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn operator_shl_80(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_81(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_82(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_83(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_84(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_85(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if !(unimplemented!()) {
        {}
        (unsafe { test_transcode_78(unimplemented!(), unimplemented!(), unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_86(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_79(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_message_87(
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
                test_concat_83(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_84(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_83(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_84(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_83(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_82(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_81(_stream, _value)
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
pub unsafe fn test_concat_message_88(
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
                test_concat_83(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_86(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_83(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_86(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_83(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_85(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_81(_stream, _value)
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
pub static mut test_89: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(|| unsafe {
    lambda_unsafe! ( Generic , { } , | mut expected : () , mut fmt : () , args : *mut test_range_format_string_std_string_ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_69 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_string_ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_90 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_string_ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_91 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_string_std_string_view_ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_92 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_string_view_ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_93 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_string_view_ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_94 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_string_std_list_char__std_allocator_char___ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_95 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_list_char__std_allocator_char___ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_96 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_list_char__std_allocator_char___ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_97 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_98 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_99 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_100 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_101 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_102 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_103 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_104 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_105 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_106 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_107 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_108 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_109 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_debug_string_std_string_ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_110 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_string_ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_111 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_string_ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_112 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_debug_string_std_string_view_ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_113 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_string_view_ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_114 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_string_view_ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_115 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_debug_string_std_list_char__std_allocator_char___ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_116 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_list_char__std_allocator_char___ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_117 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_list_char__std_allocator_char___ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_118 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_119 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_120 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_121 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_122 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_123 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_124 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_125 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_126 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_127 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_128 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_129 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_130 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_string_std_wstring_ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_131 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_wstring_ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_132 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_wstring_ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_133 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_string_std_wstring_view_ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_134 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_wstring_view_ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_135 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_wstring_view_ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_136 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_137 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_138 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_139 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_140 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_141 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_142 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_143 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_144 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_145 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_146 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_147 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_148 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_149 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_150 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_151 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_debug_string_std_wstring_ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_152 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_wstring_ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_153 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_wstring_ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_154 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_debug_string_std_wstring_view_ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_155 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_wstring_view_ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_156 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_wstring_view_ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_157 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_158 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_159 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_160 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_161 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_162 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_163 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_164 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_165 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_166 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_167 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_168 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_169 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_170 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_171 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut () = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_66 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_172 ( _condition , c"out == expected" .as_ptr() , c"format.functions.vformat.pass.cpp" .as_ptr() , 37_() , & mut _arg , ) } )  ;
 } )
});
pub static mut test_exception_173: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(
    || unsafe {
        lambda_unsafe! ( Generic , { } , | mut what : () , mut fmt : () , args : *mut test_range_format_string_std_string_ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_string_std_string_view_ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_string_std_list_char__std_allocator_char___ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_debug_string_std_string_ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_debug_string_std_string_view_ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_debug_string_std_list_char__std_allocator_char___ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_string_std_wstring_ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_string_std_wstring_view_ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_string_std_list_wchar_t__std_allocator_wchar_t___ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_debug_string_std_wstring_ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_debug_string_std_wstring_view_ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut test_range_format_debug_string_std_list_wchar_t__std_allocator_wchar_t___ , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args : *mut () , | { ;
 } , | mut what : () , mut fmt : () , args_2 : *mut () , args_3 : *mut () , | { ;
 } )
    },
);

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
        format_tests_62(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_89)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_173)).copy_from(),
        )
    });
    (unsafe {
        format_tests_63(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_89)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_173)).copy_from(),
        )
    });
    return 0_();
}
pub static mut format_kind_174: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_175: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_176: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_177: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_178: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_179: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_180: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_181: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_182: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_183: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_184: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_185: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_75);
    std::cell::LazyCell::force(&*&raw const test_89);
    std::cell::LazyCell::force(&*&raw const test_exception_173);
    std::cell::LazyCell::force(&*&raw const format_kind_174);
    std::cell::LazyCell::force(&*&raw const format_kind_175);
    std::cell::LazyCell::force(&*&raw const format_kind_176);
    std::cell::LazyCell::force(&*&raw const format_kind_177);
    std::cell::LazyCell::force(&*&raw const format_kind_178);
    std::cell::LazyCell::force(&*&raw const format_kind_179);
    std::cell::LazyCell::force(&*&raw const format_kind_180);
    std::cell::LazyCell::force(&*&raw const format_kind_181);
    std::cell::LazyCell::force(&*&raw const format_kind_182);
    std::cell::LazyCell::force(&*&raw const format_kind_183);
    std::cell::LazyCell::force(&*&raw const format_kind_184);
    std::cell::LazyCell::force(&*&raw const format_kind_185);
}
