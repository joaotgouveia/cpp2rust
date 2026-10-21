extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type widget_enum = ();
pub const widget_enum_MODE_IDLE: widget_enum = 0;
pub const widget_enum_MODE_ACTIVE: widget_enum = 1;
pub const widget_enum_MODE_DONE: widget_enum = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct widget {
    #[offset(0)]
    pub id: (),
    #[offset(4)]
    pub mode: widget_enum,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct point_struct {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct point {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl point {
    pub fn whole(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn half(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for point {
    fn default() -> Self {
        point {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct slot_union {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl slot_union {
    pub fn i(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn u(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for slot_union {
    fn default() -> Self {
        slot_union {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
pub type slot = ();
pub const slot_SLOT_A: slot = 0;
pub const slot_SLOT_B: slot = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub tag_field: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub field: Inner,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner_struct {
    #[offset(0)]
    pub typedef_field: (),
}
pub fn is_active_0(mut w: Ptr<widget>) -> () {
    return (((w.with(|__s| __s.mode) as ()) == (widget_enum_MODE_ACTIVE as i32)) as ());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let w: Value<widget> = <Value<widget>>::default();
    (*w.borrow_mut()).id = 7_();
    (*w.borrow_mut()).mode = widget_enum_MODE_ACTIVE;
    unimplemented!();
    (*w.borrow_mut()).mode = widget_enum_MODE_DONE;
    unimplemented!();
    let mut p: point_struct = <point_struct>::default();
    p.x = 3_();
    p.y = 4_();
    unimplemented!();
    let up: Value<point> = <Value<point>>::default();
    point::whole(up.as_pointer()).write(5_());
    unimplemented!();
    let b: Value<slot_union> = <Value<slot_union>>::default();
    slot_union::i(b.as_pointer()).write(9_());
    unimplemented!();
    let mut e: slot = slot_SLOT_B;
    unimplemented!();
    let mut inner_tag: Inner = <Inner>::default();
    inner_tag.tag_field = 11_();
    unimplemented!();
    let mut inner_typedef: Inner_struct = <Inner_struct>::default();
    inner_typedef.typedef_field = 22_();
    unimplemented!();
    let mut o: Outer = <Outer>::default();
    o.field.tag_field = 33_();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
