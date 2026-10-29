extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Code = ();
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut code: Code = Code_CODE_OK;
    let mut err: Code = Code_CODE_ERR;
    if (code != 0) {
        unimplemented!();
    }
    if !(code != 0) {
        unimplemented!();
    }
    if (err != 0) {
        unimplemented!();
    }
    if !(err != 0) {
        unimplemented!();
    }
    let mut t9: () = (!(code != 0) as ());
    unimplemented!();
    let mut b4: () = (code != 0);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
