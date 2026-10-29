extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Tag_enum = ();
pub const Tag_enum_T_NUM_S: Tag_enum = 0;
pub const Tag_enum_T_NUM_U: Tag_enum = 1;
pub const Tag_enum_T_TEXT: Tag_enum = 2;
pub const Tag_enum_T_FLOAT: Tag_enum = 3;
pub const Tag_enum_T_REF: Tag_enum = 4;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub text: *const (),
    pub handle: (),
    pub signed_n: (),
    pub unsigned_n: (),
    pub f: (),
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Slot {
    pub tag: Tag_enum,
    pub payload: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: Slot = <Slot>::default();
    a.tag = Tag_enum_T_NUM_S;
    a.payload.signed_n = -7_();
    unimplemented!();
    let mut b: Slot = <Slot>::default();
    b.tag = Tag_enum_T_NUM_U;
    b.payload.unsigned_n = 3735928559_();
    unimplemented!();
    let mut c: Slot = <Slot>::default();
    c.tag = Tag_enum_T_TEXT;
    c.payload.text = (c"hello".as_ptr().cast_mut()).cast_const();
    unimplemented!();
    let mut d: Slot = <Slot>::default();
    d.tag = Tag_enum_T_FLOAT;
    d.payload.f = 1.5_f64;
    unimplemented!();
    let mut x: () = 0_();
    let mut e: Slot = <Slot>::default();
    e.tag = Tag_enum_T_REF;
    e.payload.handle = ((&mut x as *mut ()) as ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
