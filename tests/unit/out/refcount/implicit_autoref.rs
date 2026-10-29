extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub v: std_vector_int__std_allocator_int__,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            v: field!(_a0, v).with_mut(std::mem::take),
        }
    }
}
pub fn write_through_0(mut p: Ptr<()>) {
    p.write(42_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(10_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(20_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    let mut p: Ptr<std_vector_int__std_allocator_int__> = (v.as_pointer());
    let mut a: () = (elem!((p), 0_()).read());
    elem!((p), 1_()).write(30_());
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(40_()));
        (*h.borrow()).v.push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(50_()));
        (*h.borrow()).v.push_back_58(___x.as_pointer())
    });
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let mut b: () = (elem!(field_ptr!((hp), v), 0_()).read());
    elem!(field_ptr!((hp), v), 1_()).write(60_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ write_through_0(({ (*p.upgrade().deref()).at_49(0_()) })) });
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
