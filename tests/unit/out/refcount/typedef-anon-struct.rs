extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_RunInfo {
    #[offset(0)]
    pub block_idx: (),
    #[offset(4)]
    pub num_extra_zero_runs: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub runs: std_vector_Outer_RunInfo__std_allocator_Outer_RunInfo__,
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        Self {
            runs: field!(_a0, runs).with_mut(std::mem::take),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    let info: Value<Outer_RunInfo> = Rc::new(RefCell::new(<Outer_RunInfo>::default()));
    (*info.borrow_mut()).block_idx = 1_();
    (*info.borrow_mut()).num_extra_zero_runs = 2_();
    ({ (*o.borrow()).runs.push_back_57(info.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field!((*self), runs).write({ { (*_a0.upgrade().deref()).runs.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
