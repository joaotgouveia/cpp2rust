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
pub struct Pod {
    #[offset(0)]
    pub v: (),
}
pub fn zero_0() -> Ptr<()> {
    return Ptr::<()>::null();
}
pub fn zero_1() -> () {
    return unimplemented!();
}
pub fn destroy_2(mut p: Ptr<()>) {}
pub fn destroy_3(mut p: Ptr<Pod>) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut i: () = unimplemented!();
    let mut d: () = unimplemented!();
    let mut p: Ptr<()> = ({ zero_0() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(5_()));
    ({ destroy_2((x.as_pointer())) });
    unimplemented!();
    let pod: Value<Pod> = Rc::new(RefCell::new(Pod { v: 7_() }));
    ({ destroy_3((pod.as_pointer())) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
