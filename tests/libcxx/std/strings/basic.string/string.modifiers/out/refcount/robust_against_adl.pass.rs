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
pub struct Charlike_Holder_Incomplete__ {
    #[offset(0)]
    pub ch_: (),
}
impl Charlike_Holder_Incomplete__ {
    pub fn new(mut ch: ()) -> Self {
        Self { ch_: ch }
    }
}
pub fn test_string_0() {
    let mut s: () = unimplemented!();
    let a: Value<Box<[Charlike_Holder_Incomplete__]>> = Rc::new(RefCell::new(Box::new([
        Charlike_Holder_Incomplete__::new({ ('m' as ()) }),
        Charlike_Holder_Incomplete__::new({ ('a' as ()) }),
        Charlike_Holder_Incomplete__::new({ ('h' as ()) }),
        Charlike_Holder_Incomplete__::new({ ('i' as ()) }),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_1() -> () {
    ({ test_string_0() });
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
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait Charlike_Holder_Incomplete__Impl {
    fn to_(&self) -> ();
}
impl Charlike_Holder_Incomplete__Impl for Ptr<Charlike_Holder_Incomplete__> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.ch_);
    }
}
pub fn __cpp2rust_init_globals() {}
