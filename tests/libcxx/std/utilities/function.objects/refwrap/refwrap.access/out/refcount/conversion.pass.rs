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
pub struct functor1 {}
pub fn test_0(t: Ptr<FnPtr<fn()>>) {
    let mut r: Ptr<FnPtr<fn()>> = (t).clone();
    let r2: Ptr<FnPtr<fn()>> = (r).clone();
    assert!(({ (r2).clone() } == { (t).clone() }));
}
pub fn test_1(t: FnPtr<fn()>) {
    let mut r: FnPtr<fn()> = (t).clone();
    let r2: FnPtr<fn()> = (r).clone();
    assert!(({ (r2).clone() } == { (t).clone() }));
}
pub fn test_2(t: Ptr<functor1>) {
    let mut r: Ptr<functor1> = (t).clone();
    let r2: Ptr<functor1> = (r).clone();
    assert!(({ (r2).clone() } == { (t).clone() }));
}
pub fn test_3(t: Ptr<i32>) {
    let mut r: Ptr<i32> = (t).clone();
    let r2: Ptr<i32> = (r).clone();
    assert!(({ (r2).clone() } == { (t).clone() }));
}
pub fn test_4(t: Ptr<i32>) {
    let mut r: Ptr<i32> = (t).clone();
    let r2: Ptr<i32> = (r).clone();
    assert!(({ (r2).clone() } == { (t).clone() }));
}
pub fn f_5() {}
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
    let fp: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(f_5)));
    ({ test_0(fp.as_pointer()) });
    ({ test_1(FnPtr::<fn()>::new(f_5)) });
    let f1: Value<functor1> = Rc::new(RefCell::new(<functor1>::default()));
    ({ test_2(f1.as_pointer()) });
    let i: Value<i32> = Rc::new(RefCell::new(0));
    ({ test_3(i.as_pointer()) });
    let j: Value<i32> = Rc::new(RefCell::new(0));
    ({ test_4(j.as_pointer()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
