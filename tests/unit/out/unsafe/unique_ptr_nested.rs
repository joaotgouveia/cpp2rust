extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub x: (),
    pub y: (),
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Outer {
    pub inner: (),
}
impl Outer {
    pub unsafe fn move_from(_a0: *mut Outer) -> Self {
        let mut this = Self {
            inner: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        unimplemented!();
        return &mut (*(self as *mut Outer));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut o: () = unimplemented!();
    (*unimplemented!()).x += 5_();
    let mut sum: () = (((*unimplemented!()).x) + ((*unimplemented!()).y));
    let mut a: () = unimplemented!();
    let mut b: () = unimplemented!();
    (*unimplemented!()) = (*unimplemented!());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
