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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool_
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_float__double__long_double_ {}
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
pub fn for_each_0(mut _a0: types_type_list_float__double__long_double_, f: TestFloat) {
    let f: Value<TestFloat> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ TestFloatImpl::operator_call_2_float(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ TestFloatImpl::operator_call_2_double(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ TestFloatImpl::operator_call_2_long_double(&f.as_pointer()) });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2)
    });
}
pub fn for_each_2(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool_,
    f: TestInt,
) {
    let f: Value<TestInt> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ TestIntImpl::operator_call_2_char(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ TestIntImpl::operator_call_2_wchar_t(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ TestIntImpl::operator_call_2_char8_t(&f.as_pointer()) });
            0_()
        };
        let _arg3: () = {
            ({ TestIntImpl::operator_call_2_char16_t(&f.as_pointer()) });
            0_()
        };
        let _arg4: () = {
            ({ TestIntImpl::operator_call_2_char32_t(&f.as_pointer()) });
            0_()
        };
        let _arg5: () = {
            ({ TestIntImpl::operator_call_2_signed_char(&f.as_pointer()) });
            0_()
        };
        let _arg6: () = {
            ({ TestIntImpl::operator_call_2_short(&f.as_pointer()) });
            0_()
        };
        let _arg7: () = {
            ({ TestIntImpl::operator_call_2_int(&f.as_pointer()) });
            0_()
        };
        let _arg8: () = {
            ({ TestIntImpl::operator_call_2_long(&f.as_pointer()) });
            0_()
        };
        let _arg9: () = {
            ({ TestIntImpl::operator_call_2_long_long(&f.as_pointer()) });
            0_()
        };
        let _arg10: () = {
            ({ TestIntImpl::operator_call_2___int128(&f.as_pointer()) });
            0_()
        };
        let _arg11: () = {
            ({ TestIntImpl::operator_call_2_unsigned_char(&f.as_pointer()) });
            0_()
        };
        let _arg12: () = {
            ({ TestIntImpl::operator_call_2_unsigned_short(&f.as_pointer()) });
            0_()
        };
        let _arg13: () = {
            ({ TestIntImpl::operator_call_2_unsigned_int(&f.as_pointer()) });
            0_()
        };
        let _arg14: () = {
            ({ TestIntImpl::operator_call_2_unsigned_long(&f.as_pointer()) });
            0_()
        };
        let _arg15: () = {
            ({ TestIntImpl::operator_call_2_unsigned_long_long(&f.as_pointer()) });
            0_()
        };
        let _arg16: () = {
            ({ TestIntImpl::operator_call_2_unsigned___int128(&f.as_pointer()) });
            0_()
        };
        let _arg17: () = {
            ({ TestIntImpl::operator_call_2__Bool(&f.as_pointer()) });
            0_()
        };
        swallow_3(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16, _arg17,
        )
    });
}
pub fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: ()) {}
pub fn swallow_3(
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFloat {}
impl TestFloat {
    pub fn test_1_float() -> () {
        unimplemented!();
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
    pub fn test_1_double() -> () {
        unimplemented!();
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
    pub fn test_1_long_double() -> () {
        unimplemented!();
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
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestInt {}
impl TestInt {
    pub fn test_1_char() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_wchar_t() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_char8_t() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_char16_t() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_char32_t() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_signed_char() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_short() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_int() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_long() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_long_long() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1___int128() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_unsigned_char() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_unsigned_short() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_unsigned_int() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_unsigned_long() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_unsigned_long_long() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1_unsigned___int128() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
    pub fn test_1__Bool() -> () {
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
        return true;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleTo_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleTo_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleTo_long_double_ {}
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
        for_each_0(
            <types_type_list_float__double__long_double_>::default(),
            <TestFloat>::default(),
        )
    });
    ({
        for_each_2 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool_  >::default()  , <TestInt  >::default()  , )
    });
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait ConvertibleTo_double_Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_double_Impl for Ptr<ConvertibleTo_double_> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait ConvertibleTo_float_Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_float_Impl for Ptr<ConvertibleTo_float_> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait ConvertibleTo_long_double_Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_long_double_Impl for Ptr<ConvertibleTo_long_double_> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait TestFloatImpl {
    fn operator_call_2_float(&self);
    fn operator_call_2_double(&self);
    fn operator_call_2_long_double(&self);
}
impl TestFloatImpl for Ptr<TestFloat> {
    fn operator_call_2_float(&self) {
        ({ TestFloat::test_1_float() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_double(&self) {
        ({ TestFloat::test_1_double() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_long_double(&self) {
        ({ TestFloat::test_1_long_double() });
        const _: () = assert!(true, "test<T>()");;
    }
}
pub trait TestIntImpl {
    fn operator_call_2_char(&self);
    fn operator_call_2_wchar_t(&self);
    fn operator_call_2_char8_t(&self);
    fn operator_call_2_char16_t(&self);
    fn operator_call_2_char32_t(&self);
    fn operator_call_2_signed_char(&self);
    fn operator_call_2_short(&self);
    fn operator_call_2_int(&self);
    fn operator_call_2_long(&self);
    fn operator_call_2_long_long(&self);
    fn operator_call_2___int128(&self);
    fn operator_call_2_unsigned_char(&self);
    fn operator_call_2_unsigned_short(&self);
    fn operator_call_2_unsigned_int(&self);
    fn operator_call_2_unsigned_long(&self);
    fn operator_call_2_unsigned_long_long(&self);
    fn operator_call_2_unsigned___int128(&self);
    fn operator_call_2__Bool(&self);
}
impl TestIntImpl for Ptr<TestInt> {
    fn operator_call_2_char(&self) {
        ({ TestInt::test_1_char() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_wchar_t(&self) {
        ({ TestInt::test_1_wchar_t() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_char8_t(&self) {
        ({ TestInt::test_1_char8_t() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_char16_t(&self) {
        ({ TestInt::test_1_char16_t() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_char32_t(&self) {
        ({ TestInt::test_1_char32_t() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_signed_char(&self) {
        ({ TestInt::test_1_signed_char() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_short(&self) {
        ({ TestInt::test_1_short() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_int(&self) {
        ({ TestInt::test_1_int() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_long(&self) {
        ({ TestInt::test_1_long() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_long_long(&self) {
        ({ TestInt::test_1_long_long() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2___int128(&self) {
        ({ TestInt::test_1___int128() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_unsigned_char(&self) {
        ({ TestInt::test_1_unsigned_char() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_unsigned_short(&self) {
        ({ TestInt::test_1_unsigned_short() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_unsigned_int(&self) {
        ({ TestInt::test_1_unsigned_int() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_unsigned_long(&self) {
        ({ TestInt::test_1_unsigned_long() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_unsigned_long_long(&self) {
        ({ TestInt::test_1_unsigned_long_long() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2_unsigned___int128(&self) {
        ({ TestInt::test_1_unsigned___int128() });
        const _: () = assert!(true, "test<T>()");;
    }
    fn operator_call_2__Bool(&self) {
        ({ TestInt::test_1__Bool() });
        const _: () = assert!(true, "test<T>()");;
    }
}
pub fn __cpp2rust_init_globals() {}
