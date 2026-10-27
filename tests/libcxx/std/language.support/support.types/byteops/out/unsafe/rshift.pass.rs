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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_float__double__long_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_void_ptr__std_nullptr_t__void__int____ {}
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
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_,
    mut f: test_functor,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { test_functor::operator_call_1_char(&f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { test_functor::operator_call_1_wchar_t(&f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { test_functor::operator_call_1_char8_t(&f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { test_functor::operator_call_1_char16_t(&f) });
            0_()
        };
        let _arg4: () = {
            (unsafe { test_functor::operator_call_1_char32_t(&f) });
            0_()
        };
        let _arg5: () = {
            (unsafe { test_functor::operator_call_1_signed_char(&f) });
            0_()
        };
        let _arg6: () = {
            (unsafe { test_functor::operator_call_1_short(&f) });
            0_()
        };
        let _arg7: () = {
            (unsafe { test_functor::operator_call_1_int(&f) });
            0_()
        };
        let _arg8: () = {
            (unsafe { test_functor::operator_call_1_long(&f) });
            0_()
        };
        let _arg9: () = {
            (unsafe { test_functor::operator_call_1_long_long(&f) });
            0_()
        };
        let _arg10: () = {
            (unsafe { test_functor::operator_call_1___int128(&f) });
            0_()
        };
        let _arg11: () = {
            (unsafe { test_functor::operator_call_1_unsigned_char(&f) });
            0_()
        };
        let _arg12: () = {
            (unsafe { test_functor::operator_call_1_unsigned_short(&f) });
            0_()
        };
        let _arg13: () = {
            (unsafe { test_functor::operator_call_1_unsigned_int(&f) });
            0_()
        };
        let _arg14: () = {
            (unsafe { test_functor::operator_call_1_unsigned_long(&f) });
            0_()
        };
        let _arg15: () = {
            (unsafe { test_functor::operator_call_1_unsigned_long_long(&f) });
            0_()
        };
        let _arg16: () = {
            (unsafe { test_functor::operator_call_1_unsigned___int128(&f) });
            0_()
        };
        swallow_1(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16,
        )
    });
}
pub unsafe fn for_each_2(
    mut _a0: types_type_list_float__double__long_double_,
    mut f: test_failing_functor,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { test_failing_functor::operator_call_1_float(&f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { test_failing_functor::operator_call_1_double(&f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { test_failing_functor::operator_call_1_long_double(&f) });
            0_()
        };
        swallow_3(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn for_each_4(
    mut _a0: types_type_list_void_ptr__std_nullptr_t__void__int____,
    mut f: test_failing_functor,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { test_failing_functor::operator_call_1_void_ptr(&f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { test_failing_functor::operator_call_1_nullptr_t(&f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { test_failing_functor::operator_call_1_void(&f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { test_failing_functor::operator_call_1_int__void_(&f) });
            0_()
        };
        swallow_5(_arg0, _arg1, _arg2, _arg3)
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
) {
}
pub unsafe fn swallow_3(mut _a0: (), mut _a1: (), mut _a2: ()) {}
pub unsafe fn swallow_5(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: ()) {}
pub static mut can_shr_byte_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut can_shr_byte_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_shr_byte_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_shr_byte_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_shr_byte_26: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_shr_byte_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_shr_byte_28: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut can_shr_byte_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_functor {}
impl test_functor {
    pub unsafe fn operator_call_1_char(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_wchar_t(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_char8_t(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_char16_t(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_char32_t(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_signed_char(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_short(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_int(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_long(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_long_long(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1___int128(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_unsigned_char(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_unsigned_short(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_unsigned_int(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_unsigned_long(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_unsigned_long_long(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_call_1_unsigned___int128(&self) {
        const _: () = assert!(true, "can_shr_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} >> I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} >> I{{}}), std::byte>"
        );;
        let mut b100: () = 100_();
        let mut b115: () = 115_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_failing_functor {}
impl test_failing_functor {
    pub unsafe fn operator_call_1_float(&self) {
        const _: () = assert!(true, "!can_shr_byte<T>");;
    }
    pub unsafe fn operator_call_1_double(&self) {
        const _: () = assert!(true, "!can_shr_byte<T>");;
    }
    pub unsafe fn operator_call_1_long_double(&self) {
        const _: () = assert!(true, "!can_shr_byte<T>");;
    }
    pub unsafe fn operator_call_1_void_ptr(&self) {
        const _: () = assert!(true, "!can_shr_byte<T>");;
    }
    pub unsafe fn operator_call_1_nullptr_t(&self) {
        const _: () = assert!(true, "!can_shr_byte<T>");;
    }
    pub unsafe fn operator_call_1_void(&self) {
        const _: () = assert!(true, "!can_shr_byte<T>");;
    }
    pub unsafe fn operator_call_1_int__void_(&self) {
        const _: () = assert!(true, "!can_shr_byte<T>");;
    }
}
pub unsafe fn test_bool_30(mut b: ()) {
    const _: () = assert!(true, "noexcept(b >> true)");;
    const _: () = assert!(true, "std::is_same_v<decltype(b >> true), std::byte>");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_31() -> () {
    (unsafe {
        for_each_0 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , <test_functor >::default() , )
    });
    (unsafe {
        for_each_2(
            <types_type_list_float__double__long_double_>::default(),
            <test_failing_functor>::default(),
        )
    });
    (unsafe {
        for_each_4(
            <types_type_list_void_ptr__std_nullptr_t__void__int____>::default(),
            <test_failing_functor>::default(),
        )
    });
    (unsafe { test_bool_30(0_()) });
    (unsafe { test_bool_30(1_()) });
    (unsafe { test_bool_30(127_()) });
    (unsafe { test_bool_30(128_()) });
    (unsafe { test_bool_30(129_()) });
    (unsafe { test_bool_30(255_()) });
    return true;
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
    (unsafe { test_31() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const can_shr_byte_6);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_7);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_8);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_9);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_10);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_11);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_12);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_13);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_14);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_15);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_16);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_17);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_18);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_19);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_20);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_21);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_22);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_23);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_24);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_25);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_26);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_27);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_28);
    std::cell::LazyCell::force(&*&raw const can_shr_byte_29);
}
