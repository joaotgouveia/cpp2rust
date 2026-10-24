extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct MoveOnly {}
impl MoveOnly {
    pub fn new() -> Self {
        Self {}
    }
    pub fn move_from(_a0: Ptr<MoveOnly>) -> Self {
        Self {}
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new() }
    }
}
pub fn make_move_only_0() -> MoveOnly {
    return MoveOnly::new();
}
pub fn test01_1() {
    let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*t1.borrow_mut()) = (*unimplemented!().upgrade().deref()).clone();
    let t4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let t5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let t6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*t4.borrow_mut()) = (*unimplemented!().upgrade().deref()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_1() });
    return 0_();
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
