extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut buf: [(); 32] = [unimplemented!(); 32];
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    buf[(0_()) as usize] = ('Z' as ());
    unimplemented!();
    unimplemented!();
    let mut fmt: [(); 8] = [unimplemented!(); 8];
    fmt[(0_()) as usize] = ('%' as ());
    fmt[(1_()) as usize] = ('5' as ());
    fmt[(2_()) as usize] = ('.' as ());
    fmt[(3_()) as usize] = ('1' as ());
    fmt[(4_()) as usize] = ('f' as ());
    fmt[(5_()) as usize] = 0_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
