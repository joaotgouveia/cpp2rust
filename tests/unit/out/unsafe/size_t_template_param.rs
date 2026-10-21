extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn array_ref_0(a: *mut [(); 3]) -> () {
    (*a)[(unimplemented!()) as usize] = ((*a)[(unimplemented!()) as usize]).wrapping_add(1_());
    return (*a)[((3_() as ()).wrapping_sub(1_())) as usize];
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrCtor_unsigned_long_ {
    pub v: (),
}
impl PtrCtor_unsigned_long_ {
    pub unsafe fn new(mut p: *const ()) -> Self {
        let mut this = Self {
            v: (*p.offset((1_()) as isize)),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RefCtor_unsigned_long_ {
    pub v: (),
}
impl RefCtor_unsigned_long_ {
    pub unsafe fn new(x: *const ()) -> Self {
        let mut this = Self {
            v: (*x).wrapping_add(1_()),
        };
        this
    }
}
pub unsafe fn neg_ll_1() -> () {
    return -9000000_();
}
pub unsafe fn neg_char_2() -> () {
    return (b'\x80' as ());
}
pub unsafe fn neg_int_3() -> () {
    return -1_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a1: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    unimplemented!();
    let mut a2: [(); 2] = [4_(), 5_()];
    let mut pc: PtrCtor_unsigned_long_ =
        PtrCtor_unsigned_long_::new({ (a2.as_mut_ptr()).cast_const() });
    unimplemented!();
    let mut v1: () = 6_();
    let mut rc: RefCtor_unsigned_long_ = RefCtor_unsigned_long_::new({ &v1 });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
