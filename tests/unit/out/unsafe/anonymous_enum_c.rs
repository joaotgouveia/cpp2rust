extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_FIRST_A: anon_0 = 0;
pub const anon_0_FIRST_B: anon_0 = 1;
pub type anon_1 = ();
pub const anon_1_SECOND_A: anon_1 = 0;
pub const anon_1_SECOND_B: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub a: (),
}
pub type TdEnum_enum = ();
pub const TdEnum_enum_TD_A: TdEnum_enum = 0;
pub const TdEnum_enum_TD_B: TdEnum_enum = 1;
pub type anon_2 = ();
pub const anon_2_FIELD_A: anon_2 = 0;
pub const anon_2_FIELD_B: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithAnonField {
    pub a: (),
    pub field: anon_2,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut td: TdEnum_enum = TdEnum_enum_TD_A;
    unimplemented!();
    td = TdEnum_enum_TD_B;
    unimplemented!();
    let mut w: WithAnonField = <WithAnonField>::default();
    w.field = anon_2_FIELD_A;
    unimplemented!();
    w.field = anon_2_FIELD_B;
    unimplemented!();
    return 0_();
}
pub type anon_3 = ();
pub const anon_3_THIRD_A: anon_3 = 0;
pub const anon_3_THIRD_B: anon_3 = 1;
pub unsafe fn __cpp2rust_init_globals() {}
