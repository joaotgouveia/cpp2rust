extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Tag_enum = ();
pub const Tag_enum_T_NUM_S: Tag_enum = 0;
pub const Tag_enum_T_NUM_U: Tag_enum = 1;
pub const Tag_enum_T_TEXT: Tag_enum = 2;
pub const Tag_enum_T_FLOAT: Tag_enum = 3;
pub const Tag_enum_T_REF: Tag_enum = 4;
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn text(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
    pub fn handle(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn signed_n(this: Ptr<Self>) -> Ptr<i64> {
        this.reinterpret_cast()
    }
    pub fn unsigned_n(this: Ptr<Self>) -> Ptr<u64> {
        this.reinterpret_cast()
    }
    pub fn f(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Slot {
    #[offset(0)]
    pub tag: Tag_enum,
    #[offset(8)]
    #[byte_size(8)]
    pub payload: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Slot> = <Value<Slot>>::default();
    (*a.borrow_mut()).tag = Tag_enum_T_NUM_S;
    anon_0::signed_n(field_ptr!(a.as_pointer(), payload)).write((-7_() as i64));
    unimplemented!();
    let b: Value<Slot> = <Value<Slot>>::default();
    (*b.borrow_mut()).tag = Tag_enum_T_NUM_U;
    anon_0::unsigned_n(field_ptr!(b.as_pointer(), payload)).write(3735928559_u64);
    unimplemented!();
    let c: Value<Slot> = <Value<Slot>>::default();
    (*c.borrow_mut()).tag = Tag_enum_T_TEXT;
    anon_0::text(field_ptr!(c.as_pointer(), payload))
        .write(Ptr::<()>::from_string_literal(b"hello"));
    unimplemented!();
    let d: Value<Slot> = <Value<Slot>>::default();
    (*d.borrow_mut()).tag = Tag_enum_T_FLOAT;
    anon_0::f(field_ptr!(d.as_pointer(), payload)).write(1.5_f64);
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(0_()));
    let e: Value<Slot> = <Value<Slot>>::default();
    (*e.borrow_mut()).tag = Tag_enum_T_REF;
    anon_0::handle(field_ptr!(e.as_pointer(), payload))
        .write(((x.as_pointer()) as Ptr<()>).to_any());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
