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
    let the_map : Value<std_map_bool__moveable__std_less_bool___std_allocator_std_pair_const_bool__moveable___ > = Rc::new(RefCell::new(<std_map_bool__moveable__std_less_bool___std_allocator_std_pair_const_bool__moveable___  >::default() )) ;
    return ({ moveableImpl::to_i32(&the_map.as_pointer().offset(true)) });
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
