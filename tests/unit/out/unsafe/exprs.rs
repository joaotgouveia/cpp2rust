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
pub struct X {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Y {
    pub x: X,
    pub p: *mut X,
}
impl Y {
    pub unsafe fn foo(&mut self) -> *mut X {
        return &mut self.x;
    }
    pub unsafe fn ptr(&mut self) -> *mut X {
        return (&mut self.x as *mut X);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x1: () = 5_();
    let mut x2: () = x1;
    let mut x3: () = ((x1) + (5_()));
    let mut x4: () = ((x3) + (x2));
    x1 = 5_();
    x2 = x1;
    x3 = ((x1) + (5_()));
    x4 = ((x3) + (x2));
    let mut p1: *mut () = (&mut x1 as *mut ());
    p1 = (&mut x2 as *mut ());
    (*p1) = x1;
    (*p1) = (((x1) + (x4)) + (1_()));
    let mut x5: () = (*p1);
    let mut x6: () = (((*p1) + (x3)) + (5_()));
    let r: *mut () = &mut x1;
    (*r) = 5_();
    (*r) = ((*p1) + (5_()));
    let mut x7: () = (*r);
    let mut x8: () = (((*r) + (x1)) + (5_()));
    let mut p2: *mut () = (r);
    let mut x: X = X { x: 1_() };
    let mut y: Y = Y {
        x: X { x: 0_() },
        p: (&mut x as *mut X),
    };
    y.x.x = 5_();
    (*(unsafe { Y::foo(&mut y) })).x = 1_();
    (*y.p).x = 10_();
    let mut p3: *mut Y = (&mut y as *mut Y);
    (*(*p3).p).x = 100_();
    (*(unsafe { Y::ptr(&mut y) })).x = 1_();
    (*(unsafe { Y::ptr(&mut y) })).x = 50_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
