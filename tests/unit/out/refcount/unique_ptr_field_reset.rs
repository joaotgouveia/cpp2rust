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
pub struct Data {
    #[offset(0)]
    pub v: (),
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub data: (),
    #[offset(8)]
    pub n: (),
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            data: unimplemented!(),
            n: { (*_a0.upgrade().deref()).n },
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    (*h.borrow_mut()).n = 1_();
    let mut hp: Ptr<Holder> = (h.as_pointer());
    unimplemented!();
    unimplemented!();
    ({ HolderImpl::set(&h.as_pointer(), Ptr::alloc(Data { v: 4_() })) });
    unimplemented!();
    let __rhs = hp.with(|__s| __s.n);
    unimplemented!().v += __rhs;
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn set(&self, p: Ptr<Data>);
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn set(&self, mut p: Ptr<Data>) {
        unimplemented!();
    }
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        unimplemented!();
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
