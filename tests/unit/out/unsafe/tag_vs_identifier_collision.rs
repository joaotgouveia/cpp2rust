extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type widget_enum = ();
pub const widget_enum_MODE_IDLE: widget_enum = 0;
pub const widget_enum_MODE_ACTIVE: widget_enum = 1;
pub const widget_enum_MODE_DONE: widget_enum = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct widget {
    pub id: (),
    pub mode: widget_enum,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct point_struct {
    pub x: (),
    pub y: (),
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union point {
    pub whole: (),
    pub half: (),
}
impl Default for point {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union slot_union {
    pub i: (),
    pub u: (),
}
impl Default for slot_union {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type slot = ();
pub const slot_SLOT_A: slot = 0;
pub const slot_SLOT_B: slot = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub tag_field: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub field: Inner,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner_struct {
    pub typedef_field: (),
}
pub unsafe fn is_active_0(mut w: *mut widget) -> () {
    return ((((*w).mode as ()) == (widget_enum_MODE_ACTIVE as i32)) as ());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut w: widget = <widget>::default();
    w.id = 7_();
    w.mode = widget_enum_MODE_ACTIVE;
    unimplemented!();
    w.mode = widget_enum_MODE_DONE;
    unimplemented!();
    let mut p: point_struct = <point_struct>::default();
    p.x = 3_();
    p.y = 4_();
    unimplemented!();
    let mut up: point = <point>::default();
    up.whole = 5_();
    unimplemented!();
    let mut b: slot_union = <slot_union>::default();
    b.i = 9_();
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
pub unsafe fn __cpp2rust_init_globals() {}
