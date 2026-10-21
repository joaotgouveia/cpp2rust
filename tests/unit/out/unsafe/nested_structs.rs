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
pub struct Level0_Level1_1_Level2_1_Level3_1 {
    pub x1: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1_Level2_1_Level3_2 {
    pub x1: (),
    pub x2: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1_Level2_1 {
    pub x1: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1 {
    pub x1: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_2 {
    pub x1: (),
    pub x2: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0 {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x1: Level0_Level1_1 = Level0_Level1_1 {
        x1: unimplemented!(),
    };
    let mut x2: Level0_Level1_2 = Level0_Level1_2 { x1: 1_(), x2: 2_() };
    let mut x3: Level0_Level1_1_Level2_1 = Level0_Level1_1_Level2_1 { x1: 3_() };
    let mut x4: Level0_Level1_1_Level2_1_Level3_1 = Level0_Level1_1_Level2_1_Level3_1 { x1: 4_() };
    let mut x5: Level0_Level1_1_Level2_1_Level3_2 =
        Level0_Level1_1_Level2_1_Level3_2 { x1: 5_(), x2: 6_() };
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
