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
pub struct A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct nothing {}
thread_local!(
    static c_0: Value<i8> = Rc::new(RefCell::new(0_i8));
);

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
    {
        let i: Value<i32> = Rc::new(RefCell::new(0_i32));
        let d: Value<f64> = Rc::new(RefCell::new(0_f64));
        assert!((unimplemented!() == (i.as_pointer())));
        assert!((unimplemented!() == (d.as_pointer())));
        let mut tp: Ptr<A> = Ptr::alloc(<A>::default());
        let mut ctp: Ptr<A> = (tp).clone();
        assert!(({ unimplemented!() } == { (tp).clone() }));
        assert!(({ unimplemented!() } == { (tp).clone() }));
        tp.delete();
    }
    {
        let anon_1: Value<anon_1> = Rc::new(RefCell::new(<anon_1>::default()));
        assert!(
            ((unimplemented!() as Ptr<nothing>).to_any()
                == (unimplemented!() as Ptr<i32>).to_any())
        );
    }
    return 0;
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl anon_1 {
    pub fn n(this: Ptr<Self>) -> Ptr<nothing> {
        this.reinterpret_cast()
    }
    pub fn i(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
pub trait AImpl {
    fn operator_addr(&self);
}
impl AImpl for Ptr<A> {
    fn operator_addr(&self) {}
}
pub trait nothingImpl {
    fn to_pmuti8(&self) -> Ptr<i8>;
}
impl nothingImpl for Ptr<nothing> {
    fn to_pmuti8(&self) -> Ptr<i8> {
        return c_0.with(|v| v.as_pointer());
    }
}
pub fn __cpp2rust_init_globals() {}
