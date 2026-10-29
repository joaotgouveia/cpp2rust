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
pub struct Item {
    pub flags: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut item: Item = Item { flags: 0_() };
    let mut ptr: *mut Item = (&mut item as *mut Item);
    (*(ptr)).flags |= ((1_()) << (0_()));
    (*(ptr)).flags |= ((1_()) << (1_()));
    unimplemented!();
    (*(ptr)).flags &= !((1_()) << (0_()));
    unimplemented!();
    let mut bits: [(); 4] = [0_(), unimplemented!(), unimplemented!(), unimplemented!()];
    (bits)[((5_()) / (8_())) as usize] |= ((1_()) << ((5_()) & (7_())));
    (bits)[((13_()) / (8_())) as usize] |= ((1_()) << ((13_()) & (7_())));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if (((*ptr).flags) != (0_())) {
        (*(ptr)).flags &= !((1_()) << (1_()));
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
