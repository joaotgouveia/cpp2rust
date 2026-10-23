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
pub struct Sink {}
impl Sink {
    pub fn new_1(mut _a0: Ptr<Sink>) -> Self {
        Self {}
    }
}
pub fn count_0(sp: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    return unimplemented!();
}
pub fn count_n_1(sp: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    return unimplemented!();
}
pub fn test_2() -> () {
    let a: Value<Box<[Sink]>> = Rc::new(RefCell::new(
        (0..10).map(|_| <Sink>::default()).collect::<Box<[Sink]>>(),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn three_3(sp: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    return unimplemented!();
}
pub fn four_4(sp: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    return unimplemented!();
}
pub fn test_P2447R4_annex_c_examples_5() -> () {
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        unimplemented!();
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
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
    unimplemented!();
    const _: () = assert!(true, "test()");;
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
