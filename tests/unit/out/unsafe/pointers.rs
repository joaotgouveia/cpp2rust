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
pub struct Test {
    pub x: (),
}
impl Test {
    pub unsafe fn inc(&mut self) {
        self.x.postfix_inc();
    }
    pub unsafe fn dec(&mut self) {
        self.x.postfix_dec();
    }
    pub unsafe fn as_ptr(&mut self) -> *mut () {
        return (&mut self.x as *mut ());
    }
    pub unsafe fn update(&mut self, mut x: (), mut y: ()) {
        self.x = ((x) + (y));
    }
}
pub unsafe fn Update_0(mut t: *mut Test) -> *mut Test {
    let mut x: () = 1_();
    let mut y: () = 2_();
    x.prefix_inc();
    (unsafe { Test::update(&mut (*t), x, y) });
    x = (*t).x;
    y = (*t).x;
    (unsafe {
        let _x: () = x;
        let _y: () = y;
        Test::update(&mut (*t), _x, _y)
    });
    return t;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut t1: Test = Test { x: 100_() };
    let mut t2: *mut Test = (unsafe { Update_0((&mut t1 as *mut Test)) });
    let mut t3: *mut Test = std::ptr::null_mut();
    t3 = t2;
    (*t3).x = 15_();
    (*(unsafe { Test::as_ptr(&mut (*t3)) })) += 10_();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
