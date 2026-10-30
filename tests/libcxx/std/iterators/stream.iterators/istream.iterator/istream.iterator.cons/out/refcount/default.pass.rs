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
pub struct S {}
impl S {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_trivial_int__true_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_trivial_char__true_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_trivial_double__true_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_trivial_S__false_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_
{}
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
    {
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let it2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*it2.borrow_mut());
    }
    ({
        test_trivial_int__true_Impl::operator_call(
            &Rc::new(RefCell::new(<test_trivial_int__true_>::default())).as_pointer(),
        )
    });
    ({
        test_trivial_char__true_Impl::operator_call(
            &Rc::new(RefCell::new(<test_trivial_char__true_>::default())).as_pointer(),
        )
    });
    ({
        test_trivial_double__true_Impl::operator_call(
            &Rc::new(RefCell::new(<test_trivial_double__true_>::default())).as_pointer(),
        )
    });
    ({
        test_trivial_S__false_Impl::operator_call(
            &Rc::new(RefCell::new(<test_trivial_S__false_>::default())).as_pointer(),
        )
    });
    ({
        test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_Impl :: operator_call ( &Rc::new(RefCell::new(<test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_  >::default() )).as_pointer() , )
    });
    return 0_();
}
pub trait test_trivial_S__false_Impl {
    fn operator_call(&self);
}
impl test_trivial_S__false_Impl for Ptr<test_trivial_S__false_> {
    fn operator_call(&self) {}
}
pub trait test_trivial_char__true_Impl {
    fn operator_call(&self);
}
impl test_trivial_char__true_Impl for Ptr<test_trivial_char__true_> {
    fn operator_call(&self) {
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*it.borrow_mut());
    }
}
pub trait test_trivial_double__true_Impl {
    fn operator_call(&self);
}
impl test_trivial_double__true_Impl for Ptr<test_trivial_double__true_> {
    fn operator_call(&self) {
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*it.borrow_mut());
    }
}
pub trait test_trivial_int__true_Impl {
    fn operator_call(&self);
}
impl test_trivial_int__true_Impl for Ptr<test_trivial_int__true_> {
    fn operator_call(&self) {
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*it.borrow_mut());
    }
}
pub trait test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_Impl
{
    fn operator_call(&self);
}
impl test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_Impl
    for Ptr<test_trivial_std_basic_string_char__std_char_traits_char___std_allocator_char____false_>
{
    fn operator_call(&self) {}
}
pub fn __cpp2rust_init_globals() {}
