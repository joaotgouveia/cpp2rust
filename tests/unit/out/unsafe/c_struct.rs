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
pub struct Point {
    pub x: (),
    pub y: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Node {
    pub value: (),
    pub next: *mut Node,
}
pub type Color = ();
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub a: (),
    pub b: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Container {
    pub inner: Inner,
    pub color: Color,
    pub count: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: Point = Point { x: 10_(), y: 20_() };
    unimplemented!();
    unimplemented!();
    let mut q: Point = p;
    q.x = 99_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut l: Line = Line {
        start: Point { x: 1_(), y: 2_() },
        end: Point { x: 3_(), y: 4_() },
    };
    unimplemented!();
    unimplemented!();
    let mut a: Node = Node {
        value: 1_(),
        next: std::ptr::null_mut(),
    };
    let mut b: Node = Node {
        value: 2_(),
        next: (&mut a as *mut Node),
    };
    unimplemented!();
    let mut c: Container = Container {
        inner: Inner { a: 5_(), b: 6_() },
        color: Color_GREEN,
        count: 42_(),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: Container = <Container>::default();
    c2.color = Color_BLUE;
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
