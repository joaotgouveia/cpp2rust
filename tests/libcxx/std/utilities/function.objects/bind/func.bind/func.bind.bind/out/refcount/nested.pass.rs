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
pub struct multiply {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct plus_one {}
pub fn test_0() -> () {
    let g: Value<std___bind_multiply__int__const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait multiplyImpl {
    fn operator_call_1_int(&self, a: (), b: ()) -> ();
}
impl multiplyImpl for Ptr<multiply> {
    fn operator_call_1_int(&self, mut a: (), mut b: ()) -> () {
        return (a * b);
    }
}
pub trait plus_oneImpl {
    fn operator_call_1_int(&self, a: ()) -> ();
}
impl plus_oneImpl for Ptr<plus_one> {
    fn operator_call_1_int(&self, mut a: ()) -> () {
        return (a + 1_());
    }
}
pub fn __cpp2rust_init_globals() {}
