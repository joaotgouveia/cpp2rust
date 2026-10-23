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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_float__double__long_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_void_ptr__std_nullptr_t__void__int____ {}
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
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_,
    f: test_functor,
) {
    let f: Value<test_functor> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ test_functorImpl::operator_call_1_char(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ test_functorImpl::operator_call_1_wchar_t(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ test_functorImpl::operator_call_1_char8_t(&f.as_pointer()) });
            0_()
        };
        let _arg3: () = {
            ({ test_functorImpl::operator_call_1_char16_t(&f.as_pointer()) });
            0_()
        };
        let _arg4: () = {
            ({ test_functorImpl::operator_call_1_char32_t(&f.as_pointer()) });
            0_()
        };
        let _arg5: () = {
            ({ test_functorImpl::operator_call_1_signed_char(&f.as_pointer()) });
            0_()
        };
        let _arg6: () = {
            ({ test_functorImpl::operator_call_1_short(&f.as_pointer()) });
            0_()
        };
        let _arg7: () = {
            ({ test_functorImpl::operator_call_1_int(&f.as_pointer()) });
            0_()
        };
        let _arg8: () = {
            ({ test_functorImpl::operator_call_1_long(&f.as_pointer()) });
            0_()
        };
        let _arg9: () = {
            ({ test_functorImpl::operator_call_1_long_long(&f.as_pointer()) });
            0_()
        };
        let _arg10: () = {
            ({ test_functorImpl::operator_call_1___int128(&f.as_pointer()) });
            0_()
        };
        let _arg11: () = {
            ({ test_functorImpl::operator_call_1_unsigned_char(&f.as_pointer()) });
            0_()
        };
        let _arg12: () = {
            ({ test_functorImpl::operator_call_1_unsigned_short(&f.as_pointer()) });
            0_()
        };
        let _arg13: () = {
            ({ test_functorImpl::operator_call_1_unsigned_int(&f.as_pointer()) });
            0_()
        };
        let _arg14: () = {
            ({ test_functorImpl::operator_call_1_unsigned_long(&f.as_pointer()) });
            0_()
        };
        let _arg15: () = {
            ({ test_functorImpl::operator_call_1_unsigned_long_long(&f.as_pointer()) });
            0_()
        };
        let _arg16: () = {
            ({ test_functorImpl::operator_call_1_unsigned___int128(&f.as_pointer()) });
            0_()
        };
        swallow_1(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16,
        )
    });
}
pub fn for_each_2(mut _a0: types_type_list_float__double__long_double_, f: test_failing_functor) {
    let f: Value<test_failing_functor> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ test_failing_functorImpl::operator_call_1_float(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ test_failing_functorImpl::operator_call_1_double(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ test_failing_functorImpl::operator_call_1_long_double(&f.as_pointer()) });
            0_()
        };
        swallow_3(_arg0, _arg1, _arg2)
    });
}
pub fn for_each_4(
    mut _a0: types_type_list_void_ptr__std_nullptr_t__void__int____,
    f: test_failing_functor,
) {
    let f: Value<test_failing_functor> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ test_failing_functorImpl::operator_call_1_void_ptr(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ test_failing_functorImpl::operator_call_1_nullptr_t(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ test_failing_functorImpl::operator_call_1_void(&f.as_pointer()) });
            0_()
        };
        let _arg3: () = {
            ({ test_failing_functorImpl::operator_call_1_int__void_(&f.as_pointer()) });
            0_()
        };
        swallow_5(_arg0, _arg1, _arg2, _arg3)
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
) {
}
pub fn swallow_3(mut _a0: (), mut _a1: (), mut _a2: ()) {}
pub fn swallow_5(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: ()) {}
thread_local!(
    pub static can_shl_byte_6: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_7: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_8: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_9: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_10: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_11: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_12: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_13: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_14: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_15: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_16: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_17: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_18: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_19: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_20: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_21: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_22: Value<()> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static can_shl_byte_23: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_shl_byte_24: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_shl_byte_25: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_shl_byte_26: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_shl_byte_27: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_shl_byte_28: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_shl_byte_29: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_functor {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_failing_functor {}
pub fn test_bool_30(mut b: ()) {
    const _: () = assert!(true, "noexcept(b << true)");;
    const _: () = assert!(true, "std::is_same_v<decltype(b << true), std::byte>");;
    unimplemented!();
    unimplemented!();
}
pub fn test_31() -> () {
    ({
        for_each_0 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_  >::default()  , <test_functor  >::default()  , )
    });
    ({
        for_each_2(
            <types_type_list_float__double__long_double_>::default(),
            <test_failing_functor>::default(),
        )
    });
    ({
        for_each_4(
            <types_type_list_void_ptr__std_nullptr_t__void__int____>::default(),
            <test_failing_functor>::default(),
        )
    });
    ({ test_bool_30(0_()) });
    ({ test_bool_30(1_()) });
    ({ test_bool_30(127_()) });
    ({ test_bool_30(128_()) });
    ({ test_bool_30(129_()) });
    ({ test_bool_30(255_()) });
    return true;
}
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
    ({ test_31() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait test_failing_functorImpl {
    fn operator_call_1_float(&self);
    fn operator_call_1_double(&self);
    fn operator_call_1_long_double(&self);
    fn operator_call_1_void_ptr(&self);
    fn operator_call_1_nullptr_t(&self);
    fn operator_call_1_void(&self);
    fn operator_call_1_int__void_(&self);
}
impl test_failing_functorImpl for Ptr<test_failing_functor> {
    fn operator_call_1_float(&self) {
        const _: () = assert!(true, "!can_shl_byte<T>");;
    }
    fn operator_call_1_double(&self) {
        const _: () = assert!(true, "!can_shl_byte<T>");;
    }
    fn operator_call_1_long_double(&self) {
        const _: () = assert!(true, "!can_shl_byte<T>");;
    }
    fn operator_call_1_void_ptr(&self) {
        const _: () = assert!(true, "!can_shl_byte<T>");;
    }
    fn operator_call_1_nullptr_t(&self) {
        const _: () = assert!(true, "!can_shl_byte<T>");;
    }
    fn operator_call_1_void(&self) {
        const _: () = assert!(true, "!can_shl_byte<T>");;
    }
    fn operator_call_1_int__void_(&self) {
        const _: () = assert!(true, "!can_shl_byte<T>");;
    }
}
pub trait test_functorImpl {
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
}
impl test_functorImpl for Ptr<test_functor> {
    fn operator_call_1_char(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_wchar_t(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_char8_t(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_char16_t(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_char32_t(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_signed_char(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_short(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_int(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_long(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_long_long(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1___int128(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_unsigned_char(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_unsigned_short(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_unsigned_int(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_unsigned_long(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_unsigned_long_long(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_1_unsigned___int128(&self) {
        const _: () = assert!(true, "can_shl_byte<I>");;
        const _: () = assert!(true, "noexcept(std::byte{{}} << I{{}})");;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(std::byte{{}} << I{{}}), std::byte>"
        );;
        let mut b1: () = 1_();
        let mut b3: () = 3_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = can_shl_byte_6.with(|_| ());
    let _ = can_shl_byte_7.with(|_| ());
    let _ = can_shl_byte_8.with(|_| ());
    let _ = can_shl_byte_9.with(|_| ());
    let _ = can_shl_byte_10.with(|_| ());
    let _ = can_shl_byte_11.with(|_| ());
    let _ = can_shl_byte_12.with(|_| ());
    let _ = can_shl_byte_13.with(|_| ());
    let _ = can_shl_byte_14.with(|_| ());
    let _ = can_shl_byte_15.with(|_| ());
    let _ = can_shl_byte_16.with(|_| ());
    let _ = can_shl_byte_17.with(|_| ());
    let _ = can_shl_byte_18.with(|_| ());
    let _ = can_shl_byte_19.with(|_| ());
    let _ = can_shl_byte_20.with(|_| ());
    let _ = can_shl_byte_21.with(|_| ());
    let _ = can_shl_byte_22.with(|_| ());
    let _ = can_shl_byte_23.with(|_| ());
    let _ = can_shl_byte_24.with(|_| ());
    let _ = can_shl_byte_25.with(|_| ());
    let _ = can_shl_byte_26.with(|_| ());
    let _ = can_shl_byte_27.with(|_| ());
    let _ = can_shl_byte_28.with(|_| ());
    let _ = can_shl_byte_29.with(|_| ());
}
