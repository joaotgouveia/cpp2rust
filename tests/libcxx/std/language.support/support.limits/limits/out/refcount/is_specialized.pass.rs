extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double_
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub fn for_each_0(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double_,
    f: Test,
) {
    let f: Value<Test> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ TestImpl::operator_call_1_char(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ TestImpl::operator_call_1_wchar_t(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ TestImpl::operator_call_1_char8_t(&f.as_pointer()) });
            0_()
        };
        let _arg3: () = {
            ({ TestImpl::operator_call_1_char16_t(&f.as_pointer()) });
            0_()
        };
        let _arg4: () = {
            ({ TestImpl::operator_call_1_char32_t(&f.as_pointer()) });
            0_()
        };
        let _arg5: () = {
            ({ TestImpl::operator_call_1_signed_char(&f.as_pointer()) });
            0_()
        };
        let _arg6: () = {
            ({ TestImpl::operator_call_1_short(&f.as_pointer()) });
            0_()
        };
        let _arg7: () = {
            ({ TestImpl::operator_call_1_int(&f.as_pointer()) });
            0_()
        };
        let _arg8: () = {
            ({ TestImpl::operator_call_1_long(&f.as_pointer()) });
            0_()
        };
        let _arg9: () = {
            ({ TestImpl::operator_call_1_long_long(&f.as_pointer()) });
            0_()
        };
        let _arg10: () = {
            ({ TestImpl::operator_call_1___int128(&f.as_pointer()) });
            0_()
        };
        let _arg11: () = {
            ({ TestImpl::operator_call_1_unsigned_char(&f.as_pointer()) });
            0_()
        };
        let _arg12: () = {
            ({ TestImpl::operator_call_1_unsigned_short(&f.as_pointer()) });
            0_()
        };
        let _arg13: () = {
            ({ TestImpl::operator_call_1_unsigned_int(&f.as_pointer()) });
            0_()
        };
        let _arg14: () = {
            ({ TestImpl::operator_call_1_unsigned_long(&f.as_pointer()) });
            0_()
        };
        let _arg15: () = {
            ({ TestImpl::operator_call_1_unsigned_long_long(&f.as_pointer()) });
            0_()
        };
        let _arg16: () = {
            ({ TestImpl::operator_call_1_unsigned___int128(&f.as_pointer()) });
            0_()
        };
        let _arg17: () = {
            ({ TestImpl::operator_call_1__Bool(&f.as_pointer()) });
            0_()
        };
        let _arg18: () = {
            ({ TestImpl::operator_call_1_float(&f.as_pointer()) });
            0_()
        };
        let _arg19: () = {
            ({ TestImpl::operator_call_1_double(&f.as_pointer()) });
            0_()
        };
        let _arg20: () = {
            ({ TestImpl::operator_call_1_long_double(&f.as_pointer()) });
            0_()
        };
        swallow_1(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16, _arg17, _arg18, _arg19, _arg20,
        )
    });
}
pub fn swallow_1(
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Test {}
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
        for_each_0 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double_  >::default()  , <Test  >::default()  , )
    });
    const _: () = assert!(
        true,
        "!std::numeric_limits<std::complex<double> >::is_specialized"
    );;
    return 0_();
}
pub trait TestImpl {
    fn operator_call_1_char(&self);
    fn operator_call_1_wchar_t(&self);
    fn operator_call_1_char8_t(&self);
    fn operator_call_1_char16_t(&self);
    fn operator_call_1_char32_t(&self);
    fn operator_call_1_signed_char(&self);
    fn operator_call_1_short(&self);
    fn operator_call_1_int(&self);
    fn operator_call_1_long(&self);
    fn operator_call_1_long_long(&self);
    fn operator_call_1___int128(&self);
    fn operator_call_1_unsigned_char(&self);
    fn operator_call_1_unsigned_short(&self);
    fn operator_call_1_unsigned_int(&self);
    fn operator_call_1_unsigned_long(&self);
    fn operator_call_1_unsigned_long_long(&self);
    fn operator_call_1_unsigned___int128(&self);
    fn operator_call_1__Bool(&self);
    fn operator_call_1_float(&self);
    fn operator_call_1_double(&self);
    fn operator_call_1_long_double(&self);
}
impl TestImpl for Ptr<Test> {
    fn operator_call_1_char(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_wchar_t(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_char8_t(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_char16_t(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_char32_t(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_signed_char(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_short(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_int(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_long(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_long_long(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1___int128(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_unsigned_char(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_unsigned_short(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_unsigned_int(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_unsigned_long(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_unsigned_long_long(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_unsigned___int128(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1__Bool(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_float(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_double(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
    fn operator_call_1_long_double(&self) {
        const _: () = assert!(true, "std::numeric_limits<T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<const T>::is_specialized");;
        const _: () = assert!(true, "std::numeric_limits<volatile T>::is_specialized");;
        const _: () = assert!(
            true,
            "std::numeric_limits<const volatile T>::is_specialized"
        );;
    }
}
pub fn __cpp2rust_init_globals() {}
