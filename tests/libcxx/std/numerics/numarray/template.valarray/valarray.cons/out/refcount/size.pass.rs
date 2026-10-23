extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub x: (),
}
impl S {
    pub fn new() -> Self {
        Self { x: 1_() }
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
thread_local!(
    pub static cnt_dtor_0: Value<()> = Rc::new(RefCell::new(0_()));
);

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
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 100_()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 100_()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 100_()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 100_()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*cnt_dtor_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = cnt_dtor_0.with(|_| ());
}
