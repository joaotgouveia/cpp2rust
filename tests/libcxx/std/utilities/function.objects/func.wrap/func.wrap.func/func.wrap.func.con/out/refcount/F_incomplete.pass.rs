extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(64)]
pub struct X {
    #[offset(16)]
    #[byte_size(48)]
    _cb: (),
}
impl X {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct IncompleteReturnType {
    #[offset(0)]
    #[byte_size(48)]
    pub fn_: (),
}
thread_local!(
    pub static called_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn test_fn_1() -> IncompleteReturnType {
    (*called_0.with(Value::clone).borrow_mut()).prefix_inc();
    let I: Value<IncompleteReturnType> = Rc::new(RefCell::new(<IncompleteReturnType>::default()));
    return (*I.borrow()).clone();
}
pub fn test_pr34298_2() {
    const _: () = assert!(
        true,
        "std::is_copy_constructible<IncompleteReturnType>::value"
    );;
    const _: () = assert!(true, "std::is_copy_assignable<IncompleteReturnType>::value");;
    {
        let X: Value<IncompleteReturnType> =
            Rc::new(RefCell::new(<IncompleteReturnType>::default()));
        unimplemented!();
        let CX: Ptr<IncompleteReturnType> = X.as_pointer();
        let mut X2: IncompleteReturnType = (*CX.upgrade().deref()).clone();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let Empty: Value<IncompleteReturnType> =
            Rc::new(RefCell::new(<IncompleteReturnType>::default()));
        let mut X2: IncompleteReturnType = (*Empty.borrow()).clone();
        unimplemented!();
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
    ({ test_pr34298_2() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = called_0.with(|_| ());
}
