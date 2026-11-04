extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct S_int_ {
    #[offset(0)]
    pub x: i32,
}
impl CppMove for S_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S_int_ {
    fn default() -> Self {
        { S_int_ { x: 0 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<S_int_> = Rc::new(RefCell::new(<S_int_>::default()));
    ({ S_int_Impl::set(&p.as_pointer(), 3) });
    ({ cpp2rust_assert_fail_0((({ f_1((p.as_pointer())) }) == 3)) });
    return 0;
}
pub fn f_1(mut p: Ptr<S_int_>) -> i32 {
    return ({ S_int_Impl::get(&p) });
}
pub trait S_int_Impl {
    fn set(&self, v: i32);
    fn get(&self) -> i32 {
        unimplemented!()
    }
}
impl S_int_Impl for Ptr<S_int_> {
    fn set(&self, mut v: i32) {
        field!((*self), x).write(v);
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.x);
    }
}
pub fn __cpp2rust_init_globals() {}
