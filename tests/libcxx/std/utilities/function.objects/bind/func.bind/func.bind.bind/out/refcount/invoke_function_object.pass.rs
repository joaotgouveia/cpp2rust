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
pub struct DummyUnaryFunction {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BadUnaryFunction {}
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
    let b: Value<std___bind_DummyUnaryFunction__BadUnaryFunction_> =
        Rc::new(RefCell::new(unimplemented!()));
    ({
        let ___args: Value<()> = Rc::new(RefCell::new(0_()));
        (*b.borrow_mut())(___args.as_pointer())
    });
    let b2: Value<std___bind_r_long__DummyUnaryFunction__BadUnaryFunction_> =
        Rc::new(RefCell::new(
            ({
                let ___f: Value<DummyUnaryFunction> =
                    Rc::new(RefCell::new(<DummyUnaryFunction>::default()));
                let ___bound_args: Value<BadUnaryFunction> =
                    Rc::new(RefCell::new(<BadUnaryFunction>::default()));
                bind_0(___f.as_pointer(), ___bound_args.as_pointer())
            }),
        ));
    ({
        let ___args: Value<()> = Rc::new(RefCell::new(0_()));
        (*b2.borrow_mut())(___args.as_pointer())
    });
    return 0_();
}
pub trait DummyUnaryFunctionImpl {
    fn operator_call_1_struct_BadUnaryFunction(&self, _a0: Ptr<BadUnaryFunction>) -> ();
}
impl DummyUnaryFunctionImpl for Ptr<DummyUnaryFunction> {
    fn operator_call_1_struct_BadUnaryFunction(&self, _a0: Ptr<BadUnaryFunction>) -> () {
        return 0_();
    }
}
pub fn __cpp2rust_init_globals() {}
