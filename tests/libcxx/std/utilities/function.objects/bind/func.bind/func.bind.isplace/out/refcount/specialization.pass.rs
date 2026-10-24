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
pub struct My2 {}
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
        let f: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: ()| -> () {
                return ((10_() * x) + 9_());
            })));
        let mut place: My2 = <My2>::default();
        let bound: Value<std___bind__lambda_at_specialization_pass_cpp_30_14__ref__My2_ref_> =
            Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let f: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: ()| -> () {
                return ((10_() * x) + 9_());
            })));
        let mut place: My2 = <My2>::default();
        let bound: Value<std___bind__lambda_at_specialization_pass_cpp_36_14__ref__const_My2_ref_> =
            Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let f: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: ()| -> () {
                return ((10_() * x) + 9_());
            })));
        let mut place: My2 = <My2>::default();
        let bound: Value<std___bind__lambda_at_specialization_pass_cpp_42_14__ref__My2_> =
            Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let f: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: ()| -> () {
                return ((10_() * x) + 9_());
            })));
        let mut place: My2 = <My2>::default();
        let bound: Value<std___bind__lambda_at_specialization_pass_cpp_48_14__ref__const_My2_> =
            Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
