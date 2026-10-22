extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_RunInfo {
    #[offset(0)]
    pub block_idx: (),
    #[offset(4)]
    pub num_extra_zero_runs: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub runs: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut o: Outer = <Outer>::default();
    let mut info: Outer_RunInfo = <Outer_RunInfo>::default();
    info.block_idx = 1_();
    info.num_extra_zero_runs = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
