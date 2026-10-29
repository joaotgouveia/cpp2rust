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
pub struct S {
    pub head: (),
    pub tail: [(); 3],
    pub buf: [(); 4],
}
pub static mut s_0: std::cell::LazyCell<S> = std::cell::LazyCell::new(|| unsafe {
    S {
        head: 5_(),
        tail: [0; 3],
        buf: [0; 4],
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((((i) < (3_())) as ()) != 0) {
        unimplemented!();
        i.postfix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((((i) < (4_())) as ()) != 0) {
        unimplemented!();
        i.postfix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const s_0);
}
