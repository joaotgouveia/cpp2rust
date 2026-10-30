extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {}
impl MoveOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut MoveOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveOnly) -> *mut MoveOnly {
        return &mut (*(self as *mut MoveOnly));
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new() }
    }
}
pub unsafe fn make_move_only_0() -> MoveOnly {
    return MoveOnly::new();
}
pub unsafe fn test01_1() {
    let mut t1: () = unimplemented!();
    let mut t2: () = unimplemented!();
    let mut t3: () = unimplemented!();
    unimplemented!();
    let mut t4: () = unimplemented!();
    let mut t5: () = unimplemented!();
    let mut t6: () = unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
