extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(1)]
pub struct Intable_Holder_Incomplete__ {}
impl CppMove for Intable_Holder_Incomplete__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    let a: Value<Box<[Ptr<Holder_Incomplete_>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<Holder_Incomplete_>::null(),
        Ptr::<Holder_Incomplete_>::null(),
    ])));
    let p: Value<Ptr<Ptr<Holder_Incomplete_>>> = Rc::new(RefCell::new(
        (a.as_pointer() as Ptr<Ptr<Holder_Incomplete_>>),
    ));
    let __rhs = libcc2rs::next_refcount(({ (*p.borrow()).clone() }), 1_isize);
    (*p.borrow_mut()) = __rhs;
    let __rhs = libcc2rs::prev_refcount(({ (*p.borrow()).clone() }), 1_isize);
    (*p.borrow_mut()) = __rhs;
    let __rhs = libcc2rs::next_refcount(({ (*p.borrow()).clone() }), 2_isize);
    (*p.borrow_mut()) = __rhs;
    let __rhs = libcc2rs::prev_refcount(({ (*p.borrow()).clone() }), 2_isize);
    (*p.borrow_mut()) = __rhs;
    ({ advance_0(p.as_pointer(), <Intable_Holder_Incomplete__>::default()) });
    &({
        let ___first: Ptr<Ptr<Holder_Incomplete_>> = (*p.borrow()).clone();
        let ___last: Ptr<Ptr<Holder_Incomplete_>> = (*p.borrow()).clone();
        distance_1(___first, ___last)
    });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Holder_Incomplete_;
pub trait Intable_Holder_Incomplete__Impl {
    fn to_i32(&self) -> i32;
}
impl Intable_Holder_Incomplete__Impl for Ptr<Intable_Holder_Incomplete__> {
    fn to_i32(&self) -> i32 {
        return 1;
    }
}
pub fn __cpp2rust_init_globals() {}
