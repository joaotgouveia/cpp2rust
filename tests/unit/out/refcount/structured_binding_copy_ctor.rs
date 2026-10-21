extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
impl Counted {
    pub fn new(mut x: (), mut y: ()) -> Self {
        Self { x: x, y: y }
    }
    pub fn copy_from(other: Ptr<Counted>) -> Self {
        let __this: Counted = Self {
            x: (other.with(|__s| __s.x) * 2_()),
            y: (other.with(|__s| __s.y) * 2_()),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted {
            x: self.x.clone(),
            y: self.y.clone(),
        }));
        Counted::copy_from(__src.as_pointer())
    }
}
pub fn make_counted_1() -> Counted {
    return Counted::new({ 5_() }, { 6_() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<Counted> = Rc::new(RefCell::new(Counted::new({ 1_() }, { 2_() })));
    let __decomp_2: Value<Counted> = Rc::new(RefCell::new(Counted::copy_from({ s.as_pointer() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_3: Ptr<Counted> = s.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_4: Ptr<Counted> = s.as_pointer();
    unimplemented!();
    unimplemented!();
    let __decomp_5: Value<Counted> = Rc::new(RefCell::new(({ make_counted_1() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub trait CountedImpl {
    fn copy_assign(&self, _a0: Ptr<Counted>) -> Ptr<Counted>;
}
impl CountedImpl for Ptr<Counted> {
    fn copy_assign(&self, _a0: Ptr<Counted>) -> Ptr<Counted> {
        field!((*self), x).write({ _a0.with(|__s| __s.x) });
        field!((*self), y).write({ _a0.with(|__s| __s.y) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
