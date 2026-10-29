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
pub struct record {
    #[offset(0)]
    pub code: (),
    #[offset(2)]
    #[byte_size(14)]
    pub pad: Box<[()]>,
}
impl Default for record {
    fn default() -> Self {
        {
            record {
                code: unimplemented!(),
                pad: (0..14).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(128)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(128)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn h(this: Ptr<Self>) -> Ptr<record> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 128]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(128)]
pub struct inner {
    #[offset(0)]
    #[byte_size(128)]
    pub view: anon_0,
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(128)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(128)]
    __bytes: Box<[u8]>,
}
impl anon_1 {
    pub fn h(this: Ptr<Self>) -> Ptr<record> {
        this.reinterpret_cast()
    }
    pub fn nested(this: Ptr<Self>) -> Ptr<inner> {
        this.reinterpret_cast()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Box::from([0u8; 128]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(144)]
pub struct Outer {
    #[offset(0)]
    pub kind: (),
    #[offset(4)]
    pub level: (),
    #[offset(8)]
    pub variant: (),
    #[offset(12)]
    pub len: (),
    #[offset(16)]
    #[byte_size(128)]
    pub body: anon_1,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let ex: Value<Outer> = <Value<Outer>>::default();
    unimplemented!();
    (*ex.borrow_mut()).kind = 2_();
    (*ex.borrow_mut()).level = 1_();
    (*ex.borrow_mut()).variant = 6_();
    (*ex.borrow_mut()).len = 16usize;
    field!(anon_1::h(field_ptr!(ex.as_pointer(), body)), code).write(2_());
    array_elem!(anon_1::h(field_ptr!(ex.as_pointer(), body)), pad, 0_()).write(('X' as ()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
