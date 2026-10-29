extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub c: (),
    #[offset(8)]
    pub x: (),
}
pub fn pack_size_0() -> () {
    return (0 as ()).wrapping_add((0 as ()));
}
pub fn pack_size_1(mut args_0: (), mut args_1: ()) -> () {
    return (2 as ()).wrapping_add((2 as ()));
}
pub fn may_throw_2() {}
pub fn no_throw_3() {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut arr: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut s: S = <S>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut may: () = false;
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
