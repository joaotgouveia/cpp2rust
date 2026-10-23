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
pub struct S {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<Box<[S]>> = Rc::new(RefCell::new(
        (0..1).map(|_| <S>::default()).collect::<Box<[S]>>(),
    ));
    let mut v: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let dv: Value<std___debug_vector_S__std_allocator_S__> = Rc::new(RefCell::new(
        std___debug_vector_S__std_allocator_S__::new_1({ 1_() }, None),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    0
}
pub fn __cpp2rust_init_globals() {}
