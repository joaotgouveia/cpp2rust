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
pub struct detail_parse_integer_impl_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_unsigned_long_long_ {}
pub fn parse_integer_0(str: Ptr<Vec<i8>>) -> i32 {
    return ({
        detail_parse_integer_impl_int_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(<detail_parse_integer_impl_int_>::default())).as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_1(str: Ptr<Vec<i8>>) -> i64 {
    return ({
        detail_parse_integer_impl_long_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(<detail_parse_integer_impl_long_>::default())).as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_2(str: Ptr<Vec<i8>>) -> i64 {
    return ({
        detail_parse_integer_impl_long_long_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_long_long_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_3(str: Ptr<Vec<i8>>) -> u32 {
    return ({
        detail_parse_integer_impl_unsigned_int_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_unsigned_int_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_4(str: Ptr<Vec<i8>>) -> u64 {
    return ({
        detail_parse_integer_impl_unsigned_long_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_unsigned_long_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_5(str: Ptr<Vec<i8>>) -> u64 {
    return ({
        detail_parse_integer_impl_unsigned_long_long_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_unsigned_long_long_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn test_signed_6() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 1_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 5_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 6_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"-12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == ((unimplemented!() + 1) as usize)));
        let mut t: i32 = ({ parse_integer_0((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i32::MAX));
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut t: i32 = ({ parse_integer_0((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i32::MIN));
    }
}
pub fn test_signed_7() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 1_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 5_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 6_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"-12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == ((unimplemented!() + 1) as usize)));
        let mut t: i64 = ({ parse_integer_1((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i64::MAX));
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut t: i64 = ({ parse_integer_1((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i64::MIN));
    }
}
pub fn test_signed_8() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 1_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 5_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 6_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"-12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == ((unimplemented!() + 1) as usize)));
        let mut t: i64 = ({ parse_integer_2((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i64::MAX));
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut t: i64 = ({ parse_integer_2((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i64::MIN));
    }
}
pub fn test_unsigned_9() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 1_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 5_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == ((unimplemented!() + 1) as usize)));
        let mut t: u32 = ({ parse_integer_3((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == u32::MAX));
    }
}
pub fn test_unsigned_10() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 1_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 5_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == ((unimplemented!() + 1) as usize)));
        let mut t: u64 = ({ parse_integer_4((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == u64::MAX));
    }
}
pub fn test_unsigned_11() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 1_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 5_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == ((unimplemented!() + 1) as usize)));
        let mut t: u64 = ({ parse_integer_5((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == unimplemented!()));
    }
}
pub fn test_float_12() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 8_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 12_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 13_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"-12345.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
}
pub fn test_float_13() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 8_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 12_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 13_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"-12345.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
}
pub fn test_float_14() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 8_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"0.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 12_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"12345.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((*s.borrow()).len() - 1) == 13_usize));
        assert!((((s.decay().offset(((*s.borrow()).len() - 1) as isize).read()) as i32) == 0));
        assert!(
            Ptr::<i8>::from_string_literal(b"-12345.000000")
                .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
        );
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_signed_6() });
    ({ test_signed_7() });
    ({ test_signed_8() });
    ({ test_unsigned_9() });
    ({ test_unsigned_10() });
    ({ test_unsigned_11() });
    ({ test_float_12() });
    ({ test_float_13() });
    ({ test_float_14() });
    return 0;
}
pub trait detail_parse_integer_impl_int_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i32;
}
impl detail_parse_integer_impl_int_Impl for Ptr<detail_parse_integer_impl_int_> {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i32 {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_long_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i64;
}
impl detail_parse_integer_impl_long_Impl for Ptr<detail_parse_integer_impl_long_> {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i64 {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_long_long_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i64;
}
impl detail_parse_integer_impl_long_long_Impl for Ptr<detail_parse_integer_impl_long_long_> {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i64 {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_unsigned_int_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u32;
}
impl detail_parse_integer_impl_unsigned_int_Impl for Ptr<detail_parse_integer_impl_unsigned_int_> {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u32 {
        return (unimplemented!() as u32);
    }
}
pub trait detail_parse_integer_impl_unsigned_long_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u64;
}
impl detail_parse_integer_impl_unsigned_long_Impl
    for Ptr<detail_parse_integer_impl_unsigned_long_>
{
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u64 {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_unsigned_long_long_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u64;
}
impl detail_parse_integer_impl_unsigned_long_long_Impl
    for Ptr<detail_parse_integer_impl_unsigned_long_long_>
{
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u64 {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
