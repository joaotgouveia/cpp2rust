extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn begin_0(_a0: Ptr<ArrayHijacker>) -> () {
    return 42_();
}
pub fn end_1(_a0: Ptr<ArrayHijacker>) -> () {
    return 42_();
}
pub fn begin_2(_a0: Ptr<ArrayHijacker>) -> () {
    return 42_();
}
pub fn end_3(_a0: Ptr<ArrayHijacker>) -> () {
    return 42_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ArrayHijacker {}
pub fn begin_4(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn end_5(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn begin_6(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn end_7(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn cbegin_8(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn cend_9(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn cbegin_10(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn cend_11(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn rbegin_12(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn rend_13(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn rbegin_14(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn rend_15(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn crbegin_16(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn crend_17(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn crbegin_18(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
pub fn crend_19(_a0: Ptr<ContainerHijacker>) -> () {
    return 42_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct ContainerHijacker {
    #[offset(0)]
    #[byte_size(8)]
    pub a_: Ptr<()>,
}
pub fn test_20() -> () {
    {
        let a: Value<Box<[ArrayHijacker]>> = Rc::new(RefCell::new(Box::new([
            <ArrayHijacker>::default(),
            <ArrayHijacker>::default(),
            <ArrayHijacker>::default(),
        ])));
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
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let c: Value<ContainerHijacker> = Rc::new(RefCell::new(ContainerHijacker {
            a_: (a.as_pointer() as Ptr<()>),
        }));
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
    ({ test_20() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait ContainerHijackerImpl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
    fn rbegin(&self) -> Ptr<()>;
    fn rend(&self) -> Ptr<()>;
}
impl ContainerHijackerImpl for Ptr<ContainerHijacker> {
    fn begin(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.a_.clone());
    }
    fn end(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.a_.clone()).offset((3_()) as isize);
    }
    fn rbegin(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.a_.clone());
    }
    fn rend(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.a_.clone()).offset((3_()) as isize);
    }
}
pub fn __cpp2rust_init_globals() {}
