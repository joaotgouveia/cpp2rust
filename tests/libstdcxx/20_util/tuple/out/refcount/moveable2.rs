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
impl CppMove for MoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
    let t1: Value<std_tuple_MoveOnly_> = Rc::new(RefCell::new(({ make_move_only_0() })));
    let t2: Value<std_tuple_MoveOnly_> = Rc::new(RefCell::new((*t1.borrow_mut()).clone()));
    let t3: Value<std_tuple_MoveOnly_> = Rc::new(RefCell::new((*t2.borrow_mut()).clone()));
    (*t1.borrow_mut()) = (*t3.borrow()).clone();
    let t4: Value<std_tuple_MoveOnly__MoveOnly_> = Rc::new(RefCell::new({
        let __tmp_0: Value<MoveOnly> = Rc::new(RefCell::new(({ make_move_only_0() })));
        let __tmp_1: Value<MoveOnly> = Rc::new(RefCell::new(({ make_move_only_0() })));
        std_tuple_MoveOnly__MoveOnly_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    let t5: Value<std_tuple_MoveOnly__MoveOnly_> =
        Rc::new(RefCell::new((*t4.borrow_mut()).clone()));
    let t6: Value<std_tuple_MoveOnly__MoveOnly_> =
        Rc::new(RefCell::new((*t5.borrow_mut()).clone()));
    (*t4.borrow_mut()) = (*t6.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_1() });
    return 0;
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
