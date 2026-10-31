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
unsafe fn main_0() -> i32 {
    let mut tmp: Vec<i32> = Vec::new();
    let mut j: i32 = 0;
    'loop_: while ((j) < (2000)) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (10000)) {
            {
                let a0_clone = i.clone();
                tmp.push(a0_clone)
            };
            i.postfix_inc();
        }
        j.postfix_inc();
    }
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
