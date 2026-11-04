extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom)]
#[cpp_into(i32, to_i32, mut_ptr)]
#[byte_size(1)]
pub struct moveable {}
impl moveable {
    pub fn move_from(_a0: Ptr<moveable>) -> Self {
        Self {}
    }
    pub fn new() -> Self {
        Self {}
    }
}
impl CppMove for moveable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for moveable {
    fn default() -> Self {
        { moveable::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let the_map: Value<Map<bool, (Value<bool>, Value<moveable>)>> =
        Rc::new(RefCell::new(Map::new()));
    return ({
        moveableImpl::to_i32(&Map::subscript_move(
            the_map.as_pointer(),
            Rc::new(RefCell::new(true)).as_pointer(),
            |__k| cpp_move!(__k),
        ))
    });
}
pub trait moveableImpl {
    fn to_i32(&self) -> i32;
}
impl moveableImpl for Ptr<moveable> {
    fn to_i32(&self) -> i32 {
        return 0;
    }
}
pub fn __cpp2rust_init_globals() {}
