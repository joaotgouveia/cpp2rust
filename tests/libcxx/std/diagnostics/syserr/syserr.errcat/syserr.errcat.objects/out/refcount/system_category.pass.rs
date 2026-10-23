extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct StaticInit {
    #[offset(0)]
    #[byte_size(8)]
    pub ec: Ptr<()>,
}
thread_local!(
    pub static foo_0: Value<StaticInit> = Rc::new(RefCell::new(<StaticInit>::default()));
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
        let e_cat1: Ptr<()> = unimplemented!();
        let e_cond: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*e_cond.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!().write(unimplemented!());
        let e_cat1: Ptr<()> = unimplemented!();
        let mut msg: () = unimplemented!();
        let mut is_newlib: () = false;
        &(is_newlib);
        unimplemented!();
        unimplemented!();
    }
    {
        (*foo_0.with(Value::clone).borrow_mut()).ec = (unimplemented!());
        let mut m: () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait StaticInitImpl {
    fn destructor(&self);
}
impl StaticInitImpl for Ptr<StaticInit> {
    fn destructor(&self) {
        let mut str: () = unimplemented!();
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = foo_0.with(|_| ());
}
