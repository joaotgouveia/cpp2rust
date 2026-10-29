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
pub struct S {
    #[offset(0)]
    pub i_: (),
}
impl S {
    pub fn new(mut i: ()) -> Self {
        Self { i_: i }
    }
}
impl CppLess for S {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SImpl::operator_lt(__lhs, __rhs.read())
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(SImpl::operator_lt(__lhs, __rhs.read())) && !(SImpl::operator_lt(__rhs, __lhs.read()))
    }
}
pub fn test_0(container: Ptr<()>) {
    {
        let c: Ptr<()> = (container).clone();
        let mut v: S = S::new({ 1_() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let c: Ptr<()> = (container).clone();
        let mut v: S = S::new({ 1_() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_1(container: Ptr<()>) {
    {
        let c: Ptr<()> = (container).clone();
        let mut v: S = S::new({ 1_() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let c: Ptr<()> = (container).clone();
        let mut v: S = S::new({ 1_() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_2(container: Ptr<()>) {
    {
        let c: Ptr<()> = (container).clone();
        let mut v: S = S::new({ 1_() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let c: Ptr<()> = (container).clone();
        let mut v: S = S::new({ 1_() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_3(container: Ptr<()>) {
    {
        let c: Ptr<()> = (container).clone();
        let mut v: S = S::new({ 1_() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let c: Ptr<()> = (container).clone();
        let mut v: S = S::new({ 1_() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_0(m.as_pointer()) });
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_1(m.as_pointer()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_2(s.as_pointer()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_3(s.as_pointer()) });
    }
    return 0_();
}
pub trait SImpl {
    fn operator_lt(&self, lhs: S) -> ();
}
impl SImpl for Ptr<S> {
    fn operator_lt(&self, mut lhs: S) -> () {
        return (lhs.i_ < (*self).with(|__s| __s.i_));
    }
}
pub fn __cpp2rust_init_globals() {}
