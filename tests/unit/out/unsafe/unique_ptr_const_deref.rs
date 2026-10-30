extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub val: (),
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            val: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        unimplemented!();
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn read_val_0(mut h: *const Holder) -> () {
    return (*unimplemented!());
}
pub unsafe fn write_val_1(mut h: *const Holder, mut v: ()) {
    (*unimplemented!()) = v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut h: Holder = <Holder>::default();
    unimplemented!();
    (unsafe { write_val_1((&mut h as *mut Holder).cast_const(), 42_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
