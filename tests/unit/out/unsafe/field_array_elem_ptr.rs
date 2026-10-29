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
pub struct Shape {
    pub id: (),
    pub coords: [(); 4],
    pub points: [Point; 3],
    pub tail: (),
}
pub unsafe fn sum_0(mut p: *const (), mut n: ()) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        s += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn set_y_1(mut p: *mut Point, mut y: ()) {
    (*p).y = y;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: Shape = Shape {
        id: 1_(),
        coords: [10_(), 20_(), 30_(), 40_()],
        points: [
            Point { x: 1_(), y: 2_() },
            Point { x: 3_(), y: 4_() },
            Point { x: 5_(), y: 6_() },
        ],
        tail: 99_(),
    };
    let mut c: *mut () = (&mut s.coords[(1_()) as usize] as *mut ());
    unimplemented!();
    (*c) = 21_();
    unimplemented!();
    c = (c).wrapping_add((2_() as ()) as usize);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *mut Point = (&mut s.points[(1_()) as usize] as *mut Point);
    unimplemented!();
    (unsafe { set_y_1(p.offset((1_()) as isize), 60_()) });
    unimplemented!();
    let mut py: *mut () = (&mut s.points[(0_()) as usize].y as *mut ());
    (*py) = 7_();
    unimplemented!();
    let mut px: *mut () = (&mut (*(p.offset((1_()) as isize))).x as *mut ());
    (*px) += 50_();
    unimplemented!();
    let mut sp: *mut Shape = (&mut s as *mut Shape);
    let mut d: *mut () = (*sp).coords.as_mut_ptr().offset((3_()) as isize);
    (*d) = 41_();
    unimplemented!();
    (*sp).points[(1_()) as usize].y =
        (((*sp).coords[(0_()) as usize]) + ((*sp).points[(0_()) as usize].x));
    unimplemented!();
    let mut q: *mut Point = (*sp).points.as_mut_ptr();
    (*q.offset((2_()) as isize)).x = 8_();
    unimplemented!();
    let mut t: Shape = s;
    t.coords[(0_()) as usize] = 0_();
    t.points[(0_()) as usize].x = 0_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
