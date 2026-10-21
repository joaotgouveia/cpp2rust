extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub a: (),
}
thread_local!(
    pub static s_0: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::<S>::null()));
);
thread_local!(
    pub static file_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static size_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {
    let _ = s_0.with(|_| ());
    let _ = file_1.with(|_| ());
    let _ = size_2.with(|_| ());
}
