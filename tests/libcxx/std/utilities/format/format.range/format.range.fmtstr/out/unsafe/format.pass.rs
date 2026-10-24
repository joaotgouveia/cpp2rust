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
pub unsafe fn get_temp_file_name_2() -> () {
    let mut Name: () = unimplemented!();
    let mut FD: () = (unsafe { mkstemp_3((&mut (*unimplemented!()) as *mut ())) });
    if ((FD) == (-1_())) {
        (unsafe { perror_4(c"mkstemp".as_ptr()) });
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn glibc_version_less_than_5(mut version: *const ()) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = (unsafe {
        confstr_6(
            (anon_7__CS_GNU_LIBC_VERSION as ()),
            std::ptr::null_mut(),
            0_(),
        )
    });
    let mut current_version: *mut () =
        Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    (unsafe { confstr_6((anon_7__CS_GNU_LIBC_VERSION as ()), current_version, n) });
    let mut result: () =
        ((unsafe { strverscmp_8((current_version).cast_const(), unimplemented!()) }) < (0_()));
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
pub unsafe fn test_format_context_create_9(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_10((*unimplemented!()), args.clone(), None) });
}
pub unsafe fn test_format_context_create_11(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_12((*unimplemented!()), args.clone(), None) });
}
pub unsafe fn test_format_13(mut expected: (), mut arg: test_range_format_string_std_string_) {
    let mut formatter: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_9(out.clone(), {
            let mut __tmp_0 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___test_range_format_string_std_string__  = ( unsafe { make_format_args_14 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_1 ( { & mut __tmp_0 } , )
        })
    });
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_string__test_range_format_string_std_string___char_ ) . format  ( & arg , & mut format_ctx , )
    });
    unimplemented!();
}
pub unsafe fn test_format_15(
    mut expected: (),
    mut arg: test_range_format_debug_string_std_string_,
) {
    let mut formatter: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_9(out.clone(), {
            let mut __tmp_1 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___test_range_format_debug_string_std_string__  = ( unsafe { make_format_args_16 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { & mut __tmp_1 } , )
        })
    });
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_debug_string__test_range_format_debug_string_std_string___char_ ) . format  ( & arg , & mut format_ctx , )
    });
    unimplemented!();
}
pub unsafe fn test_format_17(mut expected: (), mut arg: test_range_format_string_std_wstring_) {
    let mut formatter: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_11(out.clone(), {
            let mut __tmp_2 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___test_range_format_string_std_wstring__  = ( unsafe { make_format_args_18 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_1 ( { & mut __tmp_2 } , )
        })
    });
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_string__test_range_format_string_std_wstring___wchar_t_ ) . format  ( & arg , & mut format_ctx , )
    });
    unimplemented!();
}
pub unsafe fn test_format_19(
    mut expected: (),
    mut arg: test_range_format_debug_string_std_wstring_,
) {
    let mut formatter: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_11(out.clone(), {
            let mut __tmp_3 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___test_range_format_debug_string_std_wstring__  = ( unsafe { make_format_args_20 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { & mut __tmp_3 } , )
        })
    });
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_debug_string__test_range_format_debug_string_std_wstring___wchar_t_ ) . format  ( & arg , & mut format_ctx , )
    });
    unimplemented!();
}
pub unsafe fn test_fmt_21() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _arg: test_range_format_string_std_string_ =
            test_range_format_string_std_string_::new({ unimplemented!() });
        test_format_13(_expected, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _arg: test_range_format_debug_string_std_string_ =
            test_range_format_debug_string_std_string_::new({ unimplemented!() });
        test_format_15(_expected, _arg)
    });
}
pub unsafe fn test_fmt_22() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _arg: test_range_format_string_std_wstring_ =
            test_range_format_string_std_wstring_::new({ unimplemented!() });
        test_format_17(_expected, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _arg: test_range_format_debug_string_std_wstring_ =
            test_range_format_debug_string_std_wstring_::new({ unimplemented!() });
        test_format_19(_expected, _arg)
    });
}
pub unsafe fn test_23() {
    (unsafe { test_fmt_21() });
    (unsafe { test_fmt_22() });
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
    (unsafe { test_23() });
    return 0_();
}
pub static mut format_kind_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_26: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut format_kind_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const format_kind_24);
    std::cell::LazyCell::force(&*&raw const format_kind_25);
    std::cell::LazyCell::force(&*&raw const format_kind_26);
    std::cell::LazyCell::force(&*&raw const format_kind_27);
}
