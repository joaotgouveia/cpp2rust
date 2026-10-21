extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Choice_enum = ();
pub const Choice_enum_C_LIST: Choice_enum = 1;
pub const Choice_enum_C_LETTERS: Choice_enum = 2;
pub const Choice_enum_C_INTEGERS: Choice_enum = 3;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    pub items: *mut *mut (),
    pub count: i64,
    pub cursor: i64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_2 {
    pub lo: (),
    pub hi: (),
    pub curr: (),
    pub step: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_3 {
    pub lo: i64,
    pub hi: i64,
    pub curr: i64,
    pub step: i64,
    pub width: (),
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub list: anon_1,
    pub letters: anon_2,
    pub integers: anon_3,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Branch {
    pub choice: Choice_enum,
    pub index: (),
    pub v: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p_list: Branch = <Branch>::default();
    p_list.choice = Choice_enum_C_LIST;
    p_list.index = 0_();
    p_list.v.list.items = (*std::cell::LazyCell::force_mut(&mut *&raw mut items_4)).as_mut_ptr();
    p_list.v.list.count = 3_i64;
    p_list.v.list.cursor = 1_i64;
    unimplemented!();
    unimplemented!();
    let mut p_letters: Branch = <Branch>::default();
    p_letters.choice = Choice_enum_C_LETTERS;
    p_letters.index = 1_();
    p_letters.v.letters.lo = ('a' as ());
    p_letters.v.letters.hi = ('z' as ());
    p_letters.v.letters.curr = ('m' as ());
    p_letters.v.letters.step = 1_();
    unimplemented!();
    let mut p_integers: Branch = <Branch>::default();
    p_integers.choice = Choice_enum_C_INTEGERS;
    p_integers.index = 2_();
    p_integers.v.integers.lo = 1_i64;
    p_integers.v.integers.hi = 100_i64;
    p_integers.v.integers.curr = 1_i64;
    p_integers.v.integers.step = 1_i64;
    p_integers.v.integers.width = 3_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
static mut items_4: std::cell::LazyCell<[*mut (); 3]> = std::cell::LazyCell::new(|| unsafe {
    [
        c"a".as_ptr().cast_mut(),
        c"b".as_ptr().cast_mut(),
        c"c".as_ptr().cast_mut(),
    ]
});
pub unsafe fn __cpp2rust_init_globals() {}
