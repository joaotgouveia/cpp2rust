extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub v: std_vector_int__std_allocator_int__,
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self { v: (*_a0).v };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.v = ((*_a0).v).clone();
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn write_through_0(mut p: *mut ()) {
    (*p) = 42_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe {
        let mut ___x: () = 10_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 20_();
        v.push_back_58(&mut ___x)
    });
    let mut p: *mut std_vector_int__std_allocator_int__ =
        (&mut v as *mut std_vector_int__std_allocator_int__);
    let mut a: () = (&mut (*p))[(0_()) as usize];
    (&mut (*p))[(1_()) as usize] = 30_();
    let mut h: Holder = <Holder>::default();
    (unsafe {
        let mut ___x: () = 40_();
        h.v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 50_();
        h.v.push_back_58(&mut ___x)
    });
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    let mut b: () = (&mut (*hp)).v[(0_()) as usize];
    (&mut (*hp)).v[(1_()) as usize] = 60_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { write_through_0((unsafe { (*p).at_49(0_()) })) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
