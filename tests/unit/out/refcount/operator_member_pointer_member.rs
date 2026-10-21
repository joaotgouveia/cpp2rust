extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Table {}
impl Table {
    pub fn operator_index(mut i: ()) -> Ptr<()> {
        return (table_0.with(|v| v.as_pointer()) as Ptr<()>).offset(i);
    }
}
thread_local!(
    pub static table_0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([7_(), 8_(), 9_()])));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    #[byte_size(12)]
    pub data: Box<[()]>,
    #[offset(12)]
    #[byte_size(4)]
    pub inner: Inner,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                data: (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                inner: <Inner>::default(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S {
        data: Box::new([1_(), 2_(), 3_()]),
        inner: Inner { x: 9_() },
    }));
    unimplemented!();
    ({ SImpl::operator_index_1(&s.as_pointer(), 1_()) }).write(20_());
    unimplemented!();
    let cs: Ptr<S> = s.as_pointer();
    unimplemented!();
    unimplemented!();
    field!(({ SImpl::operator_deref(&s.as_pointer(),) }), x).write(10_());
    unimplemented!();
    field!(({ SImpl::operator_arrow(&s.as_pointer(),) }), x).write(11_());
    unimplemented!();
    let mut p: Ptr<()> = ({ SImpl::operator_addr(&s.as_pointer()) });
    unimplemented!();
    p.write(5_());
    unimplemented!();
    let t: Value<Table> = Rc::new(RefCell::new(<Table>::default()));
    unimplemented!();
    ({ Table::operator_index(1_()) }).write(80_());
    unimplemented!();
    return unimplemented!();
}
pub trait SImpl {
    fn operator_index_1(&self, i: ()) -> Ptr<()>;
    fn operator_index_2(&self, i: ()) -> Ptr<()>;
    fn operator_deref(&self) -> Ptr<Inner>;
    fn operator_arrow(&self) -> Ptr<Inner>;
    fn operator_addr(&self) -> Ptr<()>;
}
impl SImpl for Ptr<S> {
    fn operator_index_1(&self, mut i: ()) -> Ptr<()> {
        return (array_field_ptr!((*self), data) as Ptr<()>).offset((i) as isize);
    }
    fn operator_index_2(&self, mut i: ()) -> Ptr<()> {
        return (array_field_ptr!((*self), data) as Ptr<()>).offset((i) as isize);
    }
    fn operator_deref(&self) -> Ptr<Inner> {
        return field_ptr!((*self), inner);
    }
    fn operator_arrow(&self) -> Ptr<Inner> {
        return (field_ptr!((*self), inner));
    }
    fn operator_addr(&self) -> Ptr<()> {
        return ((array_field_ptr!((*self), data) as Ptr<()>).offset((unimplemented!()) as isize));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = table_0.with(|_| ());
}
