extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn f_void_0_1() {
    (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_void_0 {}
pub fn test_void_0_2() {
    let mut save_count: () = count_0.with(|rc| *rc.borrow());
    {
        let r1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        save_count = count_0.with(|rc| *rc.borrow());
    }
    {
        let mut fp: FnPtr<fn()> = FnPtr::<fn()>::new(f_void_0_1);
        let r1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        save_count = count_0.with(|rc| *rc.borrow());
    }
    {
        let mut a0: A_void_0 = <A_void_0>::default();
        let r1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        save_count = count_0.with(|rc| *rc.borrow());
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_void_0_2() });
    return 0_();
}
pub trait A_void_0Impl {
    fn operator_call(&self);
}
impl A_void_0Impl for Ptr<A_void_0> {
    fn operator_call(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
