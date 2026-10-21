extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn retry_0(mut n: ()) -> () {
    let mut count: () = unimplemented!();
    let mut acc: () = unimplemented!();
    goto_block!({
        '__entry: {
            count = 0_();
            acc = 0_();
        }
        'again: {
            count += 1_();
            acc += n;
            if ((((count) < (3_())) as ()) != 0) {
                goto!('again);
            }
            return acc;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
