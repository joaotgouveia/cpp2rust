extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C, align(16))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub c: (),
    pub x: (),
}
pub unsafe fn pack_size_0() -> () {
    return (0 as ()).wrapping_add((0 as ()));
}
pub unsafe fn pack_size_1(mut args_0: (), mut args_1: ()) -> () {
    return (2 as ()).wrapping_add((2 as ()));
}
pub unsafe fn may_throw_2() {}
pub unsafe fn no_throw_3() {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut arr: [(); 4] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut s: S = <S>::default();
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
    let mut may: () = false;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
