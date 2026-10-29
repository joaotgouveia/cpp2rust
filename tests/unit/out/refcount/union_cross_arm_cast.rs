extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct shape_a {
    #[offset(0)]
    pub code: (),
    #[offset(2)]
    #[byte_size(14)]
    pub pad: Box<[()]>,
}
impl Default for shape_a {
    fn default() -> Self {
        {
            shape_a {
                code: unimplemented!(),
                pad: (0..14).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(28)]
pub struct shape_b {
    #[offset(0)]
    pub code: (),
    #[offset(2)]
    pub lo: (),
    #[offset(4)]
    pub mid: (),
    #[offset(8)]
    #[byte_size(16)]
    pub fill: Box<[()]>,
    #[offset(24)]
    pub tail: (),
}
impl Default for shape_b {
    fn default() -> Self {
        {
            shape_b {
                code: unimplemented!(),
                lo: unimplemented!(),
                mid: unimplemented!(),
                fill: (0..16).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                tail: unimplemented!(),
            }
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(64)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(64)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn a(this: Ptr<Self>) -> Ptr<shape_a> {
        this.reinterpret_cast()
    }
    pub fn b(this: Ptr<Self>) -> Ptr<shape_b> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 64]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(68)]
pub struct Container {
    #[offset(0)]
    pub len: (),
    #[offset(4)]
    #[byte_size(64)]
    pub u: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let c: Value<Container> = <Value<Container>>::default();
    unimplemented!();
    field!(anon_0::a(field_ptr!(c.as_pointer(), u)), code).write(10_());
    (*c.borrow_mut()).len = 28usize;
    field!(
        ((anon_0::a(field_ptr!(c.as_pointer(), u)))
            .to_any()
            .reinterpret_cast::<shape_b>()),
        tail
    )
    .write(3735928559_());
    unimplemented!();
    unimplemented!();
    field!(anon_0::b(field_ptr!(c.as_pointer(), u)), lo).write(8080_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
