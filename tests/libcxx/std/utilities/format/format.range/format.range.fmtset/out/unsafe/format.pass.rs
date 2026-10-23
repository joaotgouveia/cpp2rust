extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_eprintf_0(
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
pub unsafe fn test_log_1(
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
        test_eprintf_0(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
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
pub unsafe fn operator_cmp_2(
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
            operator_cmp_2(
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
            operator_cmp_2(
                self as *const parse_call_validator,
                other as *const parse_call_validator,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub unsafe fn get_format_types_3() -> () {
    return unimplemented!();
}
pub unsafe fn test_format_context_create_4(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_5(out_it, args.clone(), None) });
}
pub unsafe fn test_format_context_create_6(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_7(out_it, args.clone(), None) });
}
pub unsafe fn test_format_8(mut expected: (), mut arg: ()) {
    let mut formatter: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_4(out.clone(), {
            let mut __tmp_0 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_set_int__std_less_int___std_allocator_int___  = ( unsafe { make_format_args_9 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_1 ( { & mut __tmp_0 } , )
        })
    });
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_set__std_set_int__std_less_int___std_allocator_int____char_ ) . format  ( & arg , & mut format_ctx , )
    });
    unimplemented!();
}
pub unsafe fn test_format_10(mut expected: (), mut arg: ()) {
    let mut formatter: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_6(out.clone(), {
            let mut __tmp_1 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_set_int__std_less_int___std_allocator_int___  = ( unsafe { make_format_args_11 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_1 ( { & mut __tmp_1 } , )
        })
    });
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_set__std_set_int__std_less_int___std_allocator_int____wchar_t_ ) . format  ( & arg , & mut format_ctx , )
    });
    unimplemented!();
}
pub unsafe fn test_assure_parse_is_called_12(mut fmt: ()) {
    let mut arg: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_4(out.clone(), {
            let mut __tmp_2 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_set_parse_call_validator__std_less_parse_call_validator___std_allocator_parse_call_validator___  = ( unsafe { make_format_args_13 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { & mut __tmp_2 } , )
        })
    });
    let mut formatter: () = unimplemented!();
    let mut ctx: () = unimplemented!();
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_set__std_set_parse_call_validator__std_less_parse_call_validator___std_allocator_parse_call_validator____char_ ) . parse  ( & mut ctx , )
    });
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_set__std_set_parse_call_validator__std_less_parse_call_validator___std_allocator_parse_call_validator____char_ ) . format  ( & arg , & mut format_ctx , )
    });
}
pub unsafe fn test_assure_parse_is_called_14(mut fmt: ()) {
    let mut arg: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_6(out.clone(), {
            let mut __tmp_3 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_set_parse_call_validator__std_less_parse_call_validator___std_allocator_parse_call_validator___  = ( unsafe { make_format_args_15 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { & mut __tmp_3 } , )
        })
    });
    let mut formatter: () = unimplemented!();
    let mut ctx: () = unimplemented!();
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_set__std_set_parse_call_validator__std_less_parse_call_validator___std_allocator_parse_call_validator____wchar_t_ ) . parse  ( & mut ctx , )
    });
    (unsafe {
        ( formatter as std___range_default_formatter_std_range_format_set__std_set_parse_call_validator__std_less_parse_call_validator___std_allocator_parse_call_validator____wchar_t_ ) . format  ( & arg , & mut format_ctx , )
    });
}
pub unsafe fn test_assure_parse_is_called_16() {
    let mut arg: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_4(out.clone(), {
            let mut __tmp_4 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___std_set_parse_call_validator__std_less_parse_call_validator___std_allocator_parse_call_validator___  = ( unsafe { make_format_args_13 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { & mut __tmp_4 } , )
        })
    });
    {
        let formatter: () = unimplemented!();
    }
    (unsafe { test_assure_parse_is_called_12(unimplemented!()) });
    (unsafe { test_assure_parse_is_called_12(unimplemented!()) });
    (unsafe { test_assure_parse_is_called_12(unimplemented!()) });
}
pub unsafe fn test_assure_parse_is_called_17() {
    let mut arg: () = unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_6(out.clone(), {
            let mut __tmp_5 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___std_set_parse_call_validator__std_less_parse_call_validator___std_allocator_parse_call_validator___  = ( unsafe { make_format_args_15 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { & mut __tmp_5 } , )
        })
    });
    {
        let formatter: () = unimplemented!();
    }
    (unsafe { test_assure_parse_is_called_14(unimplemented!()) });
    (unsafe { test_assure_parse_is_called_14(unimplemented!()) });
    (unsafe { test_assure_parse_is_called_14(unimplemented!()) });
}
pub unsafe fn test_fmt_18() {
    (unsafe { test_format_8(unimplemented!(), unimplemented!()) });
    (unsafe { test_assure_parse_is_called_16() });
}
pub unsafe fn test_fmt_19() {
    (unsafe { test_format_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_assure_parse_is_called_17() });
}
pub unsafe fn test_20() {
    (unsafe { test_fmt_18() });
    (unsafe { test_fmt_19() });
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
    (unsafe { test_20() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
