extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static s_0: Value<()> = Rc::new(RefCell::new(55_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_int_ {}
thread_local!(
    pub static s_1: Value<()> = Rc::new(RefCell::new(55_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_char_ {}
thread_local!(
    pub static s_2: Value<()> = Rc::new(RefCell::new(55_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_long_ {}
thread_local!(
    pub static lo_3: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static hi_4: Value<()> = Rc::new(RefCell::new(1_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_int_ {}
thread_local!(
    pub static lo_5: Value<()> = Rc::new(RefCell::new(-2_()));
);
thread_local!(
    pub static hi_6: Value<()> = Rc::new(RefCell::new(2_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Range_long_ {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    s_0.with(|rc| *rc.borrow_mut() = 22_());
    s_1.with(|rc| *rc.borrow_mut() = 33_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = s_0.with(|_| ());
    let _ = s_1.with(|_| ());
    let _ = s_2.with(|_| ());
    let _ = lo_3.with(|_| ());
    let _ = hi_4.with(|_| ());
    let _ = lo_5.with(|_| ());
    let _ = hi_6.with(|_| ());
}
