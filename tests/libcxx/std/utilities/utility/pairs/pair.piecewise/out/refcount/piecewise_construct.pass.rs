extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct A {
    #[offset(0)]
    i_: (),
    #[offset(4)]
    c_: (),
}
impl A {
    pub fn new(mut i: (), mut c: ()) -> Self {
        Self { i_: i, c_: c }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct B {
    #[offset(0)]
    d_: (),
    #[offset(8)]
    u1_: (),
    #[offset(12)]
    u2_: (),
}
impl B {
    pub fn new(mut d: (), mut u1: (), mut u2: ()) -> Self {
        Self {
            d_: d,
            u1_: u1,
            u2_: u2,
        }
    }
}
pub fn test_0() -> () {
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait AImpl {
    fn get_i(&self) -> ();
    fn get_c(&self) -> ();
}
impl AImpl for Ptr<A> {
    fn get_i(&self) -> () {
        return (*self).with(|__s| __s.i_);
    }
    fn get_c(&self) -> () {
        return (*self).with(|__s| __s.c_);
    }
}
pub trait BImpl {
    fn get_d(&self) -> ();
    fn get_u1(&self) -> ();
    fn get_u2(&self) -> ();
}
impl BImpl for Ptr<B> {
    fn get_d(&self) -> () {
        return (*self).with(|__s| __s.d_);
    }
    fn get_u1(&self) -> () {
        return (*self).with(|__s| __s.u1_);
    }
    fn get_u2(&self) -> () {
        return (*self).with(|__s| __s.u2_);
    }
}
pub fn __cpp2rust_init_globals() {}
