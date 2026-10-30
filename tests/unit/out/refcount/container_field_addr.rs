extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    pub tag: (),
    #[offset(8)]
    #[byte_size(24)]
    pub v: (),
}
impl S {
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            tag: { (*_a0.upgrade().deref()).tag },
            v: unimplemented!(),
        }
    }
}
pub fn add_0(mut v: Ptr<()>, mut k: ()) {
    unimplemented!();
}
pub fn run_1(mut h: Ptr<S>) {
    ({
        let _v: Ptr<()> = (field_ptr!(h, v));
        let _k: () = h.with(|__s| __s.tag);
        add_0(_v, _k)
    });
    let mut pv: Ptr<()> = (field_ptr!(h, v));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let local: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*local.borrow_mut()).tag = 7_();
    ({ run_1((local.as_pointer())) });
    let mut heap: Ptr<S> = Ptr::alloc(<S>::default());
    field!(heap, tag).write(7_());
    ({ run_1((heap).clone()) });
    heap.delete();
    return 0_();
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
