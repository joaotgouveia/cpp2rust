extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_insert_iterator_NoIteratorAlias___std_insert_iterator_NoIteratorAlias__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoIteratorAlias_value_type {
    #[offset(0)]
    pub x: (),
}
impl NoIteratorAlias_value_type {
    pub fn new(mut d: ()) -> Self {
        Self { x: d }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct NoIteratorAlias {
    #[offset(0)]
    #[byte_size(24)]
    pub data_: Box<[()]>,
}
impl Default for NoIteratorAlias {
    fn default() -> Self {
        {
            NoIteratorAlias {
                data_: Box::new([unimplemented!(), unimplemented!(), unimplemented!()]),
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
pub fn test_1() -> () {
    let c: Value<NoIteratorAlias> = Rc::new(RefCell::new(<NoIteratorAlias>::default()));
    let mut half: () = 0.5_f64;
    let it: Value<()> = Rc::new(RefCell::new(std_insert_iterator_NoIteratorAlias_::new(
        { c.as_pointer() },
        { (array_field_ptr!(c.as_pointer(), data_) as Ptr<()>) },
    )));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait NoIteratorAliasImpl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn insert_2_struct_NoIteratorAlias_value_type(
        &self,
        pos: Ptr<()>,
        value: NoIteratorAlias_value_type,
    ) -> Ptr<()>;
}
impl NoIteratorAliasImpl for Ptr<NoIteratorAlias> {
    fn insert_2_struct_NoIteratorAlias_value_type(
        &self,
        pos: Ptr<()>,
        value: NoIteratorAlias_value_type,
    ) -> Ptr<()> {
        let pos: Value<Ptr<()>> = Rc::new(RefCell::new(pos));
        let value: Value<NoIteratorAlias_value_type> = Rc::new(RefCell::new(value));
        const _: () = assert!(true, "std::is_same_v<T, value_type>");;
        let __rhs = ({ NoIteratorAlias_value_typeImpl::to_(&value.as_pointer()) });
        (*pos.borrow()).write(__rhs);
        return (*pos.borrow()).clone();
    }
}
pub trait NoIteratorAlias_value_typeImpl {
    fn to_(&self) -> ();
}
impl NoIteratorAlias_value_typeImpl for Ptr<NoIteratorAlias_value_type> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.x);
    }
}
pub fn __cpp2rust_init_globals() {}
