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
    let r: Value<()> = Rc::new(RefCell::new(std_reference_wrapper_void__ptr____::new({
        (t).clone()
    })));
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_1(t: FnPtr<fn()>) {
    let r: Value<()> = Rc::new(RefCell::new(std_reference_wrapper_void____::new({
        (t).clone()
    })));
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_2(t: Ptr<functor1>) {
    let r: Value<()> = Rc::new(RefCell::new(std_reference_wrapper_functor1_::new({
        (t).clone()
    })));
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_3(t: Ptr<()>) {
    let r: Value<()> = Rc::new(RefCell::new(std_reference_wrapper_int_::new({
        (t).clone()
    })));
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_4(t: Ptr<()>) {
    let r: Value<()> = Rc::new(RefCell::new(std_reference_wrapper_const_int_::new({
        (t).clone()
    })));
    let r2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let fp: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(f_5)));
    ({ test_0(fp.as_pointer()) });
    ({ test_1(FnPtr::<fn()>::new(f_5)) });
    let f1: Value<functor1> = Rc::new(RefCell::new(<functor1>::default()));
    ({ test_2(f1.as_pointer()) });
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_3(i.as_pointer()) });
    let j: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_4(j.as_pointer()) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
