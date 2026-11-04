extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct X {}
impl X {
    pub fn new() -> Self {
        Self {}
    }
}
impl Callable0<()> for X {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| XImpl::operator_call(__it))
    }
}
impl CppMove for X {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for X {
    fn default() -> Self {
        { X::new() }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    let x: Value<X> = Rc::new(RefCell::new(X::new()));
    let f: Value<std_function_void____> = Rc::new(RefCell::new(std_function_void____::new_1({
        x.as_pointer()
    })));
    return 0;
}
pub trait XImpl {
    fn operator_call(&self);
}
impl XImpl for Ptr<X> {
    fn operator_call(&self) {}
}
pub fn __cpp2rust_init_globals() {}
