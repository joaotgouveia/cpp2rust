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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_float__double__long_double_ {}
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
pub unsafe fn for_each_0(mut _a0: types_type_list_float__double__long_double_, mut f: TestFloat) {
    (unsafe {
        let _arg0: () = {
            (unsafe { TestFloat::operator_call_2_float(&mut f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { TestFloat::operator_call_2_double(&mut f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { TestFloat::operator_call_2_long_double(&mut f) });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool_,
    mut f: TestInt,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { TestInt::operator_call_2_char(&mut f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { TestInt::operator_call_2_wchar_t(&mut f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { TestInt::operator_call_2_char8_t(&mut f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { TestInt::operator_call_2_char16_t(&mut f) });
            0_()
        };
        let _arg4: () = {
            (unsafe { TestInt::operator_call_2_char32_t(&mut f) });
            0_()
        };
        let _arg5: () = {
            (unsafe { TestInt::operator_call_2_signed_char(&mut f) });
            0_()
        };
        let _arg6: () = {
            (unsafe { TestInt::operator_call_2_short(&mut f) });
            0_()
        };
        let _arg7: () = {
            (unsafe { TestInt::operator_call_2_int(&mut f) });
            0_()
        };
        let _arg8: () = {
            (unsafe { TestInt::operator_call_2_long(&mut f) });
            0_()
        };
        let _arg9: () = {
            (unsafe { TestInt::operator_call_2_long_long(&mut f) });
            0_()
        };
        let _arg10: () = {
            (unsafe { TestInt::operator_call_2___int128(&mut f) });
            0_()
        };
        let _arg11: () = {
            (unsafe { TestInt::operator_call_2_unsigned_char(&mut f) });
            0_()
        };
        let _arg12: () = {
            (unsafe { TestInt::operator_call_2_unsigned_short(&mut f) });
            0_()
        };
        let _arg13: () = {
            (unsafe { TestInt::operator_call_2_unsigned_int(&mut f) });
            0_()
        };
        let _arg14: () = {
            (unsafe { TestInt::operator_call_2_unsigned_long(&mut f) });
            0_()
        };
        let _arg15: () = {
            (unsafe { TestInt::operator_call_2_unsigned_long_long(&mut f) });
            0_()
        };
        let _arg16: () = {
            (unsafe { TestInt::operator_call_2_unsigned___int128(&mut f) });
            0_()
        };
        let _arg17: () = {
            (unsafe { TestInt::operator_call_2__Bool(&mut f) });
            0_()
        };
        swallow_3(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16, _arg17,
        )
    });
}
pub unsafe fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: ()) {}
pub unsafe fn swallow_3(
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
) {
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFloat {}
impl TestFloat {
    pub unsafe fn test_1_float() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_double() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_long_double() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn operator_call_2_float(&mut self) {
        (unsafe { TestFloat::test_1_float() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_double(&mut self) {
        (unsafe { TestFloat::test_1_double() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_long_double(&mut self) {
        (unsafe { TestFloat::test_1_long_double() });
        const _: () = assert!(true, "test<T>()");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestInt {}
impl TestInt {
    pub unsafe fn test_1_char() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_wchar_t() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_char8_t() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_char16_t() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_char32_t() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_signed_char() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_short() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_int() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_long() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_long_long() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1___int128() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_unsigned_char() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_unsigned_short() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_unsigned_int() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_unsigned_long() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_unsigned_long_long() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1_unsigned___int128() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn test_1__Bool() -> () {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return true;
    }
    pub unsafe fn operator_call_2_char(&mut self) {
        (unsafe { TestInt::test_1_char() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_wchar_t(&mut self) {
        (unsafe { TestInt::test_1_wchar_t() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_char8_t(&mut self) {
        (unsafe { TestInt::test_1_char8_t() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_char16_t(&mut self) {
        (unsafe { TestInt::test_1_char16_t() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_char32_t(&mut self) {
        (unsafe { TestInt::test_1_char32_t() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_signed_char(&mut self) {
        (unsafe { TestInt::test_1_signed_char() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_short(&mut self) {
        (unsafe { TestInt::test_1_short() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_int(&mut self) {
        (unsafe { TestInt::test_1_int() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_long(&mut self) {
        (unsafe { TestInt::test_1_long() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_long_long(&mut self) {
        (unsafe { TestInt::test_1_long_long() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2___int128(&mut self) {
        (unsafe { TestInt::test_1___int128() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_unsigned_char(&mut self) {
        (unsafe { TestInt::test_1_unsigned_char() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_unsigned_short(&mut self) {
        (unsafe { TestInt::test_1_unsigned_short() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_unsigned_int(&mut self) {
        (unsafe { TestInt::test_1_unsigned_int() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_unsigned_long(&mut self) {
        (unsafe { TestInt::test_1_unsigned_long() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_unsigned_long_long(&mut self) {
        (unsafe { TestInt::test_1_unsigned_long_long() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2_unsigned___int128(&mut self) {
        (unsafe { TestInt::test_1_unsigned___int128() });
        const _: () = assert!(true, "test<T>()");;
    }
    pub unsafe fn operator_call_2__Bool(&mut self) {
        (unsafe { TestInt::test_1__Bool() });
        const _: () = assert!(true, "test<T>()");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_float_ {}
impl ConvertibleTo_float_ {
    pub unsafe fn to_(&self) -> () {
        return 1_();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_double_ {}
impl ConvertibleTo_double_ {
    pub unsafe fn to_(&self) -> () {
        return 1_();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleTo_long_double_ {}
impl ConvertibleTo_long_double_ {
    pub unsafe fn to_(&self) -> () {
        return 1_();
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
        for_each_0(
            <types_type_list_float__double__long_double_>::default(),
            <TestFloat>::default(),
        )
    });
    (unsafe {
        for_each_2 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool_ >::default() , <TestInt >::default() , )
    });
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
