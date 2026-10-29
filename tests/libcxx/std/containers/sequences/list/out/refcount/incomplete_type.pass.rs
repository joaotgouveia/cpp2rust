extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(56)]
pub struct A {
    #[offset(0)]
    #[byte_size(24)]
    pub l: (),
    #[offset(24)]
    #[byte_size(8)]
    pub it: (),
    #[offset(32)]
    #[byte_size(8)]
    pub cit: (),
    #[offset(40)]
    #[byte_size(8)]
    pub rit: (),
    #[offset(48)]
    #[byte_size(8)]
    pub crit: (),
}
impl A {
    pub fn move_from(_a0: Ptr<A>) -> Self {
        Self {
            l: unimplemented!(),
            it: field!(_a0, it).with_mut(std::mem::take),
            cit: field!(_a0, cit).with_mut(std::mem::take),
            rit: unimplemented!(),
            crit: unimplemented!(),
        }
    }
}
pub fn test_0() -> () {
    let mut a: A = <A>::default();
    &(a);
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
    unimplemented!();
    return 0_();
}
pub trait AImpl {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
