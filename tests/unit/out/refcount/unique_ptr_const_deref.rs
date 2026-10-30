extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub val: (),
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            val: unimplemented!(),
        }
    }
}
pub fn read_val_0(mut h: Ptr<Holder>) -> () {
    return (unimplemented!().read());
}
pub fn write_val_1(mut h: Ptr<Holder>, mut v: ()) {
    unimplemented!().write(v);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    unimplemented!();
    ({ write_val_1((h.as_pointer()), 42_()) });
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
