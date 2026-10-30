extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn no_args_0() -> Ptr<Holder_Incomplete_> {
    return Ptr::<Holder_Incomplete_>::null();
}
pub fn one_arg_1(p: Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn two_args_2(
    p: Ptr<Holder_Incomplete_>,
    mut _a1: Ptr<Holder_Incomplete_>,
) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn three_args_3(
    p: Ptr<Holder_Incomplete_>,
    mut _a1: Ptr<Holder_Incomplete_>,
    mut _a2: Ptr<Holder_Incomplete_>,
) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn one_arg_void_4(mut _a0: Ptr<Holder_Incomplete_>) {}
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
    let x: Value<Ptr<Holder_Incomplete_>> =
        Rc::new(RefCell::new(Ptr::<Holder_Incomplete_>::null()));
    let cx: Value<Ptr<Holder_Incomplete_>> =
        Rc::new(RefCell::new(Ptr::<Holder_Incomplete_>::null()));
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Holder_Incomplete_;
pub fn __cpp2rust_init_globals() {}
