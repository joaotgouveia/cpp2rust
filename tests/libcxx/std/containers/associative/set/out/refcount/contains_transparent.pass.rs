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
pub struct Comp {}
pub fn test_0() {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_1() {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test_2() -> () {
    ({ test_0() });
    {
        ({ test_1() });
    }
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
    ({ test_2() });
    return 0_();
}
pub trait CompImpl {
    fn operator_call_1(&self, lhs: Ptr<()>, rhs: Ptr<()>) -> ();
    fn operator_call_2(&self, lhs: Ptr<()>, rhs: ()) -> ();
    fn operator_call_3(&self, lhs: (), rhs: Ptr<()>) -> ();
}
impl CompImpl for Ptr<Comp> {
    fn operator_call_1(&self, lhs: Ptr<()>, rhs: Ptr<()>) -> () {
        return unimplemented!();
    }
    fn operator_call_2(&self, lhs: Ptr<()>, mut rhs: ()) -> () {
        return ({ lhs.with(|__s| __s.first) } < { rhs });
    }
    fn operator_call_3(&self, mut lhs: (), rhs: Ptr<()>) -> () {
        return ({ lhs } < { rhs.with(|__s| __s.first) });
    }
}
pub fn __cpp2rust_init_globals() {}
