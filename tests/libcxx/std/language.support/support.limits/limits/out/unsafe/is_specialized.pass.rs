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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double_,
    mut f: Test,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { Test::operator_call_1_char(&mut f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { Test::operator_call_1_wchar_t(&mut f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { Test::operator_call_1_char8_t(&mut f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { Test::operator_call_1_char16_t(&mut f) });
            0_()
        };
        let _arg4: () = {
            (unsafe { Test::operator_call_1_char32_t(&mut f) });
            0_()
        };
        let _arg5: () = {
            (unsafe { Test::operator_call_1_signed_char(&mut f) });
            0_()
        };
        let _arg6: () = {
            (unsafe { Test::operator_call_1_short(&mut f) });
            0_()
        };
        let _arg7: () = {
            (unsafe { Test::operator_call_1_int(&mut f) });
            0_()
        };
        let _arg8: () = {
            (unsafe { Test::operator_call_1_long(&mut f) });
            0_()
        };
        let _arg9: () = {
            (unsafe { Test::operator_call_1_long_long(&mut f) });
            0_()
        };
        let _arg10: () = {
            (unsafe { Test::operator_call_1___int128(&mut f) });
            0_()
        };
        let _arg11: () = {
            (unsafe { Test::operator_call_1_unsigned_char(&mut f) });
            0_()
        };
        let _arg12: () = {
            (unsafe { Test::operator_call_1_unsigned_short(&mut f) });
            0_()
        };
        let _arg13: () = {
            (unsafe { Test::operator_call_1_unsigned_int(&mut f) });
            0_()
        };
        let _arg14: () = {
            (unsafe { Test::operator_call_1_unsigned_long(&mut f) });
            0_()
        };
        let _arg15: () = {
            (unsafe { Test::operator_call_1_unsigned_long_long(&mut f) });
            0_()
        };
        let _arg16: () = {
            (unsafe { Test::operator_call_1_unsigned___int128(&mut f) });
            0_()
        };
        let _arg17: () = {
            (unsafe { Test::operator_call_1__Bool(&mut f) });
            0_()
        };
        let _arg18: () = {
            (unsafe { Test::operator_call_1_float(&mut f) });
            0_()
        };
        let _arg19: () = {
            (unsafe { Test::operator_call_1_double(&mut f) });
            0_()
        };
        let _arg20: () = {
            (unsafe { Test::operator_call_1_long_double(&mut f) });
            0_()
        };
        swallow_1(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16, _arg17, _arg18, _arg19, _arg20,
        )
    });
}
pub unsafe fn swallow_1(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
    mut _a7: (),
    mut _a8: (),
    mut _a9: (),
    mut _a10: (),
    mut _a11: (),
    mut _a12: (),
    mut _a13: (),
    mut _a14: (),
    mut _a15: (),
    mut _a16: (),
    mut _a17: (),
    mut _a18: (),
    mut _a19: (),
    mut _a20: (),
) {
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test {}
impl Test {
    pub unsafe fn operator_call_1_char(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_wchar_t(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_char8_t(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_char16_t(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_char32_t(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_signed_char(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_short(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_int(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_long(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_long_long(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1___int128(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_unsigned_char(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_unsigned_short(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_unsigned_int(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_unsigned_long(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_unsigned_long_long(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_unsigned___int128(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1__Bool(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_float(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_double(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    pub unsafe fn operator_call_1_long_double(&mut self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
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
    (unsafe {
        for_each_0 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double_ >::default() , <Test >::default() , )
    });
    const _: () = assert!(
        true,
        "!std::numeric_limits<std::complex<double> >::is_specialized"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
