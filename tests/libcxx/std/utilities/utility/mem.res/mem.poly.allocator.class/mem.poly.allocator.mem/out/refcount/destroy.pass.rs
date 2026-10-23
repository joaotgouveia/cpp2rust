extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_void__void_ {}
thread_local!(
    pub static count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(1)]
pub struct destroyable {}
impl destroyable {
    pub fn new() -> Self {
        let __this: destroyable = Self {};
        (*count_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for destroyable {
    fn default() -> Self {
        { destroyable::new() }
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
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..1).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut ptr: Ptr<destroyable> = Ptr::alloc(destroyable::new());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait destroyableImpl {
    fn destructor(&self);
}
impl destroyableImpl for Ptr<destroyable> {
    fn destructor(&self) {
        (*count_1.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_1.with(|_| ());
}
