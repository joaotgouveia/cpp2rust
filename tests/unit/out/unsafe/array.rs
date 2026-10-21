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
    let N: () = 5_();
    let mut arr1: [(); 5] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let arr2: [(); 5] = [1_(), 1_(), 1_(), 1_(), 1_()];
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (N)) {
        arr1[(i) as usize] = ((i) + (arr2[(i) as usize]));
        i.prefix_inc();
    }
    let mut fatorial: () = 1_();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (N)) {
        fatorial *= arr1[(i) as usize];
        i.prefix_inc();
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
