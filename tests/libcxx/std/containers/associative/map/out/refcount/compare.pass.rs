extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
impl Key {
    pub fn new(_a0: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Key {}
impl CppLess for Key {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        KeyImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Key {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(KeyImpl::operator_lt(__lhs, __rhs.clone()))
            && !(KeyImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
pub fn test_0() -> () {
    {
        let m_empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m_contains: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!().write(42_());
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let result2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
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
    ({ test_0() });
    return 0_();
}
pub trait KeyImpl {
    fn operator_lt(&self, _a0: Ptr<Key>) -> ();
}
impl KeyImpl for Ptr<Key> {
    fn operator_lt(&self, _a0: Ptr<Key>) -> () {
        return false;
    }
}
pub fn __cpp2rust_init_globals() {}
