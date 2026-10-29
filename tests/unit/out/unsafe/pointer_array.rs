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
pub struct StackArray {
    pub arr: [*mut (); 3],
}
pub unsafe fn IncrementAll_0(s: *mut StackArray) {
    let mut i: () = 0_();
    'loop_: while ((i) < (3_())) {
        (*(*s).arr[(i) as usize]) += 1_();
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 0_();
    let mut s: StackArray = StackArray {
        arr: [
            (&mut x as *mut ()),
            (&mut x as *mut ()),
            (&mut x as *mut ()),
        ],
    };
    (unsafe { IncrementAll_0(&mut s) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
