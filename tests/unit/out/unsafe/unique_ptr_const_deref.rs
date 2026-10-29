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
    pub val: std_unique_ptr_int__std_default_delete_int__,
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self { val: (*_a0).val };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.val = ((*_a0).val).clone();
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn read_val_0(mut h: *const Holder) -> () {
    return (*(*h).val);
}
pub unsafe fn write_val_1(mut h: *const Holder, mut v: ()) {
    (*(*h).val) = v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut h: Holder = <Holder>::default();
    h.val = (unsafe {
        let mut ___args: () = 10_();
        make_unique_2(&mut ___args)
    });
    (unsafe { write_val_1((&mut h as *mut Holder).cast_const(), 42_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
