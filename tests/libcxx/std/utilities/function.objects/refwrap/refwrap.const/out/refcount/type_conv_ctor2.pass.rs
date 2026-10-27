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
pub struct B {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A1 {
    #[offset(0)]
    #[byte_size(1)]
    pub b_: B,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A2 {
    #[offset(0)]
    #[byte_size(1)]
    pub b_: B,
}
pub fn test_0() -> () {
    {
        let a: Value<A1> = Rc::new(RefCell::new(<A1>::default()));
        const _: () = assert!(true, "");;
        let b1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        const _: () = assert!(true, "");;
        (*b1.borrow_mut()) = unimplemented!();
        unimplemented!();
    }
    {
        let a: Value<A2> = Rc::new(RefCell::new(<A2>::default()));
        const _: () = assert!(true, "");;
        let b2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        const _: () = assert!(true, "");;
        (*b2.borrow_mut()) = unimplemented!();
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
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait A1Impl {
    fn to_pmutB(&self) -> Ptr<B>;
}
impl A1Impl for Ptr<A1> {
    fn to_pmutB(&self) -> Ptr<B> {
        return field_ptr!((*self), b_);
    }
}
pub trait A2Impl {
    fn to_pmutB(&self) -> Ptr<B>;
}
impl A2Impl for Ptr<A2> {
    fn to_pmutB(&self) -> Ptr<B> {
        return field_ptr!((*self), b_);
    }
}
pub fn __cpp2rust_init_globals() {}
