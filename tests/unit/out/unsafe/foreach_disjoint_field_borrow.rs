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
pub struct S {
    pub v: std_vector_int__std_allocator_int__,
    pub a: (),
}
impl S {
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            v: (*_a0).v,
            a: (*_a0).a,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        self.v = ((*_a0).v).clone();
        self.a = (*_a0).a;
        return &mut (*(self as *mut S));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = <S>::default();
    (unsafe {
        let mut ___x: () = 1_();
        s.v.push_back_58(&mut ___x)
    });
    'loop_: for e in IterRange::new((unsafe { s.v.begin_26() }), (unsafe { s.v.end_28() })) {
        let mut e: () = (*e);
        s.a.postfix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
