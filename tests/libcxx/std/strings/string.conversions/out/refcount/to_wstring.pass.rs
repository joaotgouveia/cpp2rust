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
pub fn parse_integer_0(str: Ptr<()>) -> () {
    return ({
        detail_parse_integer_impl_int_Impl::operator_call_1_wchar_t(
            &Rc::new(RefCell::new(<detail_parse_integer_impl_int_>::default())).as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_1(str: Ptr<()>) -> () {
    return ({
        detail_parse_integer_impl_long_Impl::operator_call_1_wchar_t(
            &Rc::new(RefCell::new(<detail_parse_integer_impl_long_>::default())).as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_2(str: Ptr<()>) -> () {
    return ({
        detail_parse_integer_impl_long_long_Impl::operator_call_1_wchar_t(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_long_long_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_3(str: Ptr<()>) -> () {
    return ({
        detail_parse_integer_impl_unsigned_int_Impl::operator_call_1_wchar_t(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_unsigned_int_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_4(str: Ptr<()>) -> () {
    return ({
        detail_parse_integer_impl_unsigned_long_Impl::operator_call_1_wchar_t(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_unsigned_long_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_5(str: Ptr<()>) -> () {
    return ({
        detail_parse_integer_impl_unsigned_long_long_Impl::operator_call_1_wchar_t(
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
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut t: () = ({ parse_integer_0(s.as_pointer()) });
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut t: () = ({ parse_integer_0(s.as_pointer()) });
        unimplemented!();
    }
}
pub fn test_signed_7() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut t: () = ({ parse_integer_1(s.as_pointer()) });
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut t: () = ({ parse_integer_1(s.as_pointer()) });
        unimplemented!();
    }
}
pub fn test_signed_8() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut t: () = ({ parse_integer_2(s.as_pointer()) });
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut t: () = ({ parse_integer_2(s.as_pointer()) });
        unimplemented!();
    }
}
pub fn test_unsigned_9() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut t: () = ({ parse_integer_3(s.as_pointer()) });
        unimplemented!();
    }
}
pub fn test_unsigned_10() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut t: () = ({ parse_integer_4(s.as_pointer()) });
        unimplemented!();
    }
}
pub fn test_unsigned_11() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut t: () = ({ parse_integer_5(s.as_pointer()) });
        unimplemented!();
    }
}
pub fn test_float_12() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_float_13() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_float_14() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_signed_6() });
    ({ test_signed_7() });
    ({ test_signed_8() });
    ({ test_unsigned_9() });
    ({ test_unsigned_10() });
    ({ test_unsigned_11() });
    ({ test_float_12() });
    ({ test_float_13() });
    ({ test_float_14() });
    return 0_();
}
pub trait detail_parse_integer_impl_int_Impl {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> ();
}
impl detail_parse_integer_impl_int_Impl for Ptr<detail_parse_integer_impl_int_> {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> () {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_long_Impl {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> ();
}
impl detail_parse_integer_impl_long_Impl for Ptr<detail_parse_integer_impl_long_> {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> () {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_long_long_Impl {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> ();
}
impl detail_parse_integer_impl_long_long_Impl for Ptr<detail_parse_integer_impl_long_long_> {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> () {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_unsigned_int_Impl {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> ();
}
impl detail_parse_integer_impl_unsigned_int_Impl for Ptr<detail_parse_integer_impl_unsigned_int_> {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> () {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_unsigned_long_Impl {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> ();
}
impl detail_parse_integer_impl_unsigned_long_Impl
    for Ptr<detail_parse_integer_impl_unsigned_long_>
{
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> () {
        return unimplemented!();
    }
}
pub trait detail_parse_integer_impl_unsigned_long_long_Impl {
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> ();
}
impl detail_parse_integer_impl_unsigned_long_long_Impl
    for Ptr<detail_parse_integer_impl_unsigned_long_long_>
{
    fn operator_call_1_wchar_t(&self, str: Ptr<()>) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
