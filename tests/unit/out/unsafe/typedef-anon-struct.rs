extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_RunInfo {
    pub block_idx: (),
    pub num_extra_zero_runs: (),
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub runs: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut o: Outer = <Outer>::default();
    let mut info: Outer_RunInfo = <Outer_RunInfo>::default();
    info.block_idx = 1_();
    info.num_extra_zero_runs = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
