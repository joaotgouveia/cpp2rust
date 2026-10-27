extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn __asan_default_options_0() -> Ptr<()> {
    return Ptr::<()>::from_string_literal(
        b"check_initialization_order=true:strict_init_order=true",
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Global {}
impl Global {
    pub fn new() -> Self {
        let __this: Global = Self {};
        unimplemented!();
        __this
    }
}
impl Default for Global {
    fn default() -> Self {
        { Global::new() }
    }
}
thread_local!(
    pub static global_1: Value<Global> = Rc::new(RefCell::new(Global::new()));
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = global_1.with(|_| ());
}
