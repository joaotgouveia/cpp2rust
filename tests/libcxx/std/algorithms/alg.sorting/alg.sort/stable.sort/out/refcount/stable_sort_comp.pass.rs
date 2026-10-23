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
pub struct indirect_less {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct first_only {}
pub fn generate_sawtooth_0(mut N: (), mut M: ()) -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut x: () = 0_();
    let mut ver: () = 0_();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let __rhs = unimplemented!();
        unimplemented!().write(__rhs);
        if (x.prefix_inc() == M) {
            x = 0_();
            ver.prefix_inc();
        }
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn test_1() -> () {
    let mut N: () = 1000_();
    let mut M: () = 10_();
    {
        let v: Value<()> = Rc::new(RefCell::new(({ generate_sawtooth_0(N, M) })));
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let v: Value<()> = Rc::new(RefCell::new(({ generate_sawtooth_0(N, M) })));
            let randomness: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut i: () = 0_();
            'loop_: while (i < (N - M)) {
                unimplemented!();
                i += M;
            }
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            unimplemented!();
            i.prefix_inc();
        }
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
    ({ test_1() });
    return 0_();
}
pub trait first_onlyImpl {
    fn operator_call(&self, x: Ptr<()>, y: Ptr<()>) -> ();
}
impl first_onlyImpl for Ptr<first_only> {
    fn operator_call(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        return ({ x.with(|__s| __s.first) } < { y.with(|__s| __s.first) });
    }
}
pub trait indirect_lessImpl {
    fn operator_call_1_class_std_unique_ptr_int_(&self, x: Ptr<()>, y: Ptr<()>) -> ();
}
impl indirect_lessImpl for Ptr<indirect_less> {
    fn operator_call_1_class_std_unique_ptr_int_(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        return ({ ((*x.upgrade().deref()).read()) } < { ((*y.upgrade().deref()).read()) });
    }
}
pub fn __cpp2rust_init_globals() {}
