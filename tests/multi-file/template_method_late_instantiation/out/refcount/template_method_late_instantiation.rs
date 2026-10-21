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
    pub x: (),
}
impl Default for S_int_ {
    fn default() -> Self {
        {
            S_int_ {
                x: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let p: Value<S_int_> = Rc::new(RefCell::new(<S_int_>::default()));
    ({ S_int_Impl::set(&p.as_pointer(), 3_()) });
    unimplemented!();
    return unimplemented!();
}
pub fn f_0(mut p: Ptr<S_int_>) -> () {
    return ({ S_int_Impl::get(&p) });
}
pub trait S_int_Impl {
    fn set(&self, v: ());
    fn get(&self) -> () {
        unimplemented!()
    }
}
impl S_int_Impl for Ptr<S_int_> {
    fn set(&self, mut v: ()) {
        field!((*self), x).write(v);
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.x);
    }
}
pub fn __cpp2rust_init_globals() {}
