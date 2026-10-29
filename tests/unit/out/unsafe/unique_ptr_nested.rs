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
    pub inner: std_unique_ptr_Inner__std_default_delete_Inner__,
}
impl Outer {
    pub unsafe fn move_from(_a0: *mut Outer) -> Self {
        let mut this = Self {
            inner: (*_a0).inner,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        self.inner = ((*_a0).inner).clone();
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
    let mut o: std_unique_ptr_Outer__std_default_delete_Outer__ = (unsafe {
        let mut ___args: Outer = Outer {
            inner: (unsafe {
                let mut ___args: Inner = Inner { x: 10_(), y: 20_() };
                make_unique_0(&mut ___args)
            }),
        };
        make_unique_1(&mut ___args)
    });
    (*(*o).inner).x += 5_();
    let mut sum: () = (((*(*o).inner).x) + ((*(*o).inner).y));
    let mut a: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 100_();
        make_unique_2(&mut ___args)
    });
    let mut b: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 0_();
        make_unique_2(&mut ___args)
    });
    (*b) = (*a);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
