extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Level0_Level1_1_Level2_1_Level3_1 {
    #[offset(0)]
    pub x1: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Level0_Level1_1_Level2_1_Level3_2 {
    #[offset(0)]
    pub x1: (),
    #[offset(4)]
    pub x2: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Level0_Level1_1_Level2_1 {
    #[offset(0)]
    pub x1: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Level0_Level1_1 {
    #[offset(0)]
    pub x1: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Level0_Level1_2 {
    #[offset(0)]
    pub x1: (),
    #[offset(4)]
    pub x2: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Level0 {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
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
pub fn __cpp2rust_init_globals() {}
