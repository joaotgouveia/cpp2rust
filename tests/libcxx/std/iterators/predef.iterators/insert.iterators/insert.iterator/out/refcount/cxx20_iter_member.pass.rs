extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_insert_iterator_NoIteratorAlias___std_insert_iterator_NoIteratorAlias__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(f64, new, value)]
#[cpp_into(f64, to_f64)]
#[byte_size(4)]
pub struct NoIteratorAlias_value_type {
    #[offset(0)]
    pub x: i32,
}
impl NoIteratorAlias_value_type {
    pub fn new(mut d: f64) -> Self {
        Self { x: (d as i32) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct NoIteratorAlias {
    #[offset(0)]
    #[byte_size(24)]
    pub data_: Box<[f64]>,
}
impl Default for NoIteratorAlias {
    fn default() -> Self {
        {
            NoIteratorAlias {
                data_: Box::new([0_f64, 0_f64, 0_f64]),
            }
        }
    }
}
const _: () = assert!(
    true,
    "std::is_constructible_v<std::insert_iterator<NoIteratorAlias>, NoIteratorAlias&, double*>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::insert_iterator<NoIteratorAlias>, NoIteratorAlias&, NoIteratorAlias::value_type*>"
);
pub fn test_1() -> bool {
    let c: Value<NoIteratorAlias> = Rc::new(RefCell::new(<NoIteratorAlias>::default()));
    let mut half: f64 = 0.5_f64;
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    assert!(((array_elem!(c.as_pointer(), data_, 0).read()) == 1_f64));
    assert!(((array_elem!(c.as_pointer(), data_, 1).read()) == 2_f64));
    assert!(((array_elem!(c.as_pointer(), data_, 2).read()) == 0_f64));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait NoIteratorAliasImpl {
    fn begin(&self) -> Ptr<f64> {
        unimplemented!()
    }
    fn insert_2_struct_NoIteratorAlias_value_type(
        &self,
        pos: Ptr<f64>,
        value: NoIteratorAlias_value_type,
    ) -> Ptr<f64>;
}
impl NoIteratorAliasImpl for Ptr<NoIteratorAlias> {
    fn insert_2_struct_NoIteratorAlias_value_type(
        &self,
        pos: Ptr<f64>,
        value: NoIteratorAlias_value_type,
    ) -> Ptr<f64> {
        let pos: Value<Ptr<f64>> = Rc::new(RefCell::new(pos));
        let value: Value<NoIteratorAlias_value_type> = Rc::new(RefCell::new(value));
        const _: () = assert!(true, "std::is_same_v<T, value_type>");;
        let __rhs = ({ NoIteratorAlias_value_typeImpl::to_f64(&value.as_pointer()) });
        (*pos.borrow()).write(__rhs);
        return (*pos.borrow()).clone();
    }
}
pub trait NoIteratorAlias_value_typeImpl {
    fn to_f64(&self) -> f64;
}
impl NoIteratorAlias_value_typeImpl for Ptr<NoIteratorAlias_value_type> {
    fn to_f64(&self) -> f64 {
        return ((*self).with(|__s| __s.x) as f64);
    }
}
pub fn __cpp2rust_init_globals() {}
