extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct foo {
    #[offset(0)]
    val_: (),
}
impl foo {
    pub fn new() -> Self {
        Self { val_: 3_() }
    }
}
impl Default for foo {
    fn default() -> Self {
        { foo::new() }
    }
}
pub fn test_0() -> () {
    {
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 5_()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 5_()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let p3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 7_()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
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
pub trait fooImpl {
    fn get(&self) -> ();
}
impl fooImpl for Ptr<foo> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.val_);
    }
}
pub fn __cpp2rust_init_globals() {}
