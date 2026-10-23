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
pub struct NonConstCopyable {
    #[offset(0)]
    pub value: (),
}
impl NonConstCopyable {
    pub fn new_1(mut v: ()) -> Self {
        Self { value: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BlowsUpOnConstCopy_int_ {}
impl BlowsUpOnConstCopy_int_ {}
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
    let v: Value<NonConstCopyable> = Rc::new(RefCell::new(NonConstCopyable::new_1({ 42_() })));
    let b: Value<BlowsUpOnConstCopy_int_> =
        Rc::new(RefCell::new(<BlowsUpOnConstCopy_int_>::default()));
    let t: Value<()> = Rc::new(RefCell::new(
        std_tuple_NonConstCopyable__BlowsUpOnConstCopy_int__::new({ v.as_pointer() }, {
            b.as_pointer()
        }),
    ));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
