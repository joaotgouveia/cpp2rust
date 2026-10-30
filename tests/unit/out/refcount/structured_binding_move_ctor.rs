extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moves_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct Movable {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
impl Movable {
    pub fn new(mut x: (), mut y: ()) -> Self {
        Self { x: x, y: y }
    }
    pub fn copy_from(other: Ptr<Movable>) -> Self {
        let __this: Movable = Self {
            x: other.with(|__s| __s.x),
            y: other.with(|__s| __s.y),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(other: Ptr<Movable>) -> Self {
        let __this: Movable = Self {
            x: other.with(|__s| __s.x),
            y: other.with(|__s| __s.y),
        };
        field!(other, x).write(0_());
        field!(other, y).write(0_());
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Movable {
    fn clone(&self) -> Self {
        let __src: Value<Movable> = Rc::new(RefCell::new(Movable {
            x: self.x.clone(),
            y: self.y.clone(),
        }));
        Movable::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub xs: (),
    #[offset(24)]
    #[byte_size(24)]
    pub ys: (),
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            xs: unimplemented!(),
            ys: unimplemented!(),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let m: Value<Movable> = Rc::new(RefCell::new(Movable::new({ 3_() }, { 4_() })));
    let __decomp_2: Value<Movable> = Rc::new(RefCell::new(Movable::move_from({ m.as_pointer() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        xs: unimplemented!(),
        ys: unimplemented!(),
    }));
    let __decomp_3: Value<Holder> =
        Rc::new(RefCell::new(({ HolderImpl::extract(&h.as_pointer()) })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn extract(&self) -> Holder;
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn extract(&self) -> Holder {
        return Holder {
            xs: unimplemented!(),
            ys: unimplemented!(),
        };
    }
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
