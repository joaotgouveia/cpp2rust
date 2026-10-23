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
pub struct Intable_Holder_Incomplete__ {}
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
    let a: Value<Box<[Ptr<Holder_Incomplete_>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<Holder_Incomplete_>::null(),
        Ptr::<Holder_Incomplete_>::null(),
    ])));
    let p: Value<Ptr<Ptr<Holder_Incomplete_>>> = Rc::new(RefCell::new(
        (a.as_pointer() as Ptr<Ptr<Holder_Incomplete_>>),
    ));
    let __rhs = ({ next_0((*p.borrow()).clone(), None) });
    (*p.borrow_mut()) = __rhs;
    let __rhs = ({ prev_1((*p.borrow()).clone()) });
    (*p.borrow_mut()) = __rhs;
    let __rhs = ({ next_0((*p.borrow()).clone(), Some(2_())) });
    (*p.borrow_mut()) = __rhs;
    let __rhs = ({ prev_2((*p.borrow()).clone(), 2_()) });
    (*p.borrow_mut()) = __rhs;
    ({ advance_3(p.as_pointer(), <Intable_Holder_Incomplete__>::default()) });
    &(unimplemented!());
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Holder_Incomplete_;
pub trait Intable_Holder_Incomplete__Impl {
    fn to_(&self) -> ();
}
impl Intable_Holder_Incomplete__Impl for Ptr<Intable_Holder_Incomplete__> {
    fn to_(&self) -> () {
        return 1_();
    }
}
pub fn __cpp2rust_init_globals() {}
