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
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct nothing {}
impl CppMove for nothing {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
        assert!((({ addressof_1(i.as_pointer(),) }) == (i.as_pointer())));
        assert!((({ addressof_2(d.as_pointer(),) }) == (d.as_pointer())));
        let mut tp: Ptr<A> = Ptr::alloc(<A>::default());
        let mut ctp: Ptr<A> = (tp).clone();
        assert!(({ ({ addressof_3((tp).clone(),) }) } == { (tp).clone() }));
        assert!(({ ({ addressof_4((ctp).clone(),) }) } == { (tp).clone() }));
        tp.delete();
    }
    {
        let anon_5: Value<anon_5> = Rc::new(RefCell::new(<anon_5>::default()));
        assert!(
            ((({ addressof_6((anon_5::n(anon_5.as_pointer())).clone(),) }) as Ptr<nothing>)
                .to_any()
                == (({ addressof_1((anon_5::i(anon_5.as_pointer())).clone(),) }) as Ptr<i32>)
                    .to_any())
        );
    }
    return 0;
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct anon_5 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl anon_5 {
    pub fn n(this: Ptr<Self>) -> Ptr<nothing> {
        this.reinterpret_cast()
    }
    pub fn i(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
}
impl CppMove for anon_5 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for anon_5 {
    fn default() -> Self {
        anon_5 {
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
