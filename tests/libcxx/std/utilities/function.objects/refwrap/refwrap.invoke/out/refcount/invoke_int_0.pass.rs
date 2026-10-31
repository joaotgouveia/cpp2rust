extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn f_int_0_1() -> i32 {
    return 3;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_int_0 {}
pub fn test_int_0_2() {
    {
        let mut r1: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(f_int_0_1);
        assert!((unimplemented!() == 3));
    }
    {
        let fp: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(f_int_0_1)));
        let mut r1: Ptr<FnPtr<fn() -> i32>> = fp.as_pointer();
        assert!((unimplemented!() == 3));
    }
    {
        let a0: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
        let mut r1: Ptr<A_int_0> = a0.as_pointer();
        assert!((unimplemented!() == 4));
    }
}
pub fn f_void_1_3(mut i: i32) {
    count_0.with(|rc| *rc.borrow_mut() += i);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_void_1 {}
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
    ({ test_int_0_2() });
    return 0;
}
pub trait A_int_0Impl {
    fn operator_call(&self) -> i32;
}
impl A_int_0Impl for Ptr<A_int_0> {
    fn operator_call(&self) -> i32 {
        return 4;
    }
}
pub trait A_void_1Impl {
    fn operator_call(&self, i: i32);
}
impl A_void_1Impl for Ptr<A_void_1> {
    fn operator_call(&self, mut i: i32) {
        count_0.with(|rc| *rc.borrow_mut() += i);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
