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
pub struct Pointer {
    #[offset(0)]
    #[byte_size(8)]
    pub value: (),
}
impl Pointer {
    pub fn new(mut v: ()) -> Self {
        Self { value: (v).clone() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub n: (),
}
impl A {
    pub fn new() -> Self {
        Self { n: 42_() }
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
thread_local!(
    pub static i_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static d_1: Value<()> = Rc::new(RefCell::new(0_f64));
);
thread_local!(
    pub static a_2: Value<A> = Rc::new(RefCell::new(A::new()));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    const _: () = assert!(true, "std::addressof(i) == &i");;
    const _: () = assert!(true, "std::addressof(d) == &d");;
    let mut ap: Ptr<A> = unimplemented!();
    const _: () = assert!(true, "&ap->n == &a.n");;
    return 0_();
}
pub trait AImpl {
    fn operator_addr(&self);
}
impl AImpl for Ptr<A> {
    fn operator_addr(&self) {}
}
pub fn __cpp2rust_init_globals() {
    let _ = i_0.with(|_| ());
    let _ = d_1.with(|_| ());
    let _ = a_2.with(|_| ());
}
