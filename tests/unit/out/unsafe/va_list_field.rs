extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0(mut n: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut guard: Guard_1 = Guard_1::new({ &mut ap });
    let _dtor_guard = ScopedDestructorUnsafe::new(&raw mut guard, Guard_1::destructor);
    let mut total: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        total += (*guard.ap).arg::<()>();
        i.prefix_inc();
    }
    return total;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Guard_1 {
    pub ap: *mut *const VaArg,
    pub active: (),
}
impl Guard_1 {
    pub unsafe fn new(val: *mut *const VaArg) -> Self {
        let mut this = Self {
            ap: val,
            active: true,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        if self.active {}
    }
}
pub unsafe fn sum_ptr_2(mut n: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut cursor: Cursor_3 = Cursor_3 {
        ap: (&mut ap as *mut *const VaArg),
    };
    let mut total: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        total += (*cursor.ap).arg::<()>();
        i.prefix_inc();
    }
    return total;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cursor_3 {
    pub ap: *mut *const VaArg,
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
