extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveAssign)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub a: (),
    #[offset(8)]
    pub k: (),
}
impl S {
    pub fn new() -> Self {
        Self {
            a: unimplemented!(),
            k: (unimplemented!().read()),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
pub fn getP_0() -> () {
    return unimplemented!();
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
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!().write(5_());
        unimplemented!().write(6_());
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0_();
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        unimplemented!();
        field!((*self), k).write({ { (*_a0.upgrade().deref()).k } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
