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
    pub tag: (),
    pub v: std_vector_int__std_allocator_int__,
}
impl S {
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            tag: (*_a0).tag,
            v: (*_a0).v,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        self.tag = (*_a0).tag;
        self.v = ((*_a0).v).clone();
        return &mut (*(self as *mut S));
    }
}
pub unsafe fn add_0(mut v: *mut std_vector_int__std_allocator_int__, mut k: ()) {
    (unsafe { (*v).push_back_57(&k) });
}
pub unsafe fn run_1(mut h: *mut S) {
    (unsafe {
        let _v: *mut std_vector_int__std_allocator_int__ =
            (&mut (*h).v as *mut std_vector_int__std_allocator_int__);
        let _k: () = (*h).tag;
        add_0(_v, _k)
    });
    let mut pv: *mut std_vector_int__std_allocator_int__ =
        (&mut (*h).v as *mut std_vector_int__std_allocator_int__);
    (unsafe {
        let mut ___x: () = (unsafe { (*(pv).cast_const()).size() });
        (*pv).push_back_58(&mut ___x)
    });
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut local: S = <S>::default();
    local.tag = 7_();
    (unsafe { run_1((&mut local as *mut S)) });
    let mut heap: *mut S = (Box::leak(Box::new(<S>::default())) as *mut S);
    (*heap).tag = 7_();
    (unsafe { run_1(heap) });
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
