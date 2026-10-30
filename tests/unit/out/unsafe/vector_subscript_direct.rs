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
impl Point {
    pub unsafe fn sum(&self) -> () {
        return ((self.x) + (self.y));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub values: (),
    pub points: (),
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            values: unimplemented!(),
            points: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn push_and_index_0(v: *mut ()) -> () {
    unimplemented!();
    return ((unimplemented!()) - (1_()));
}
pub unsafe fn sum_ref_1(v: *const ()) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        s += (&(*v))[(i) as usize];
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: () = unimplemented!();
    v[(0_()) as usize] = 10_();
    unimplemented!();
    v[(1_()) as usize] += 5_();
    v[(2_()) as usize].postfix_inc();
    unimplemented!();
    v[(0_()) as usize] = v[(1_()) as usize];
    unimplemented!();
    v[(1_()) as usize] = 0_();
    unimplemented!();
    let mut i: () = 0_();
    v[(i.postfix_inc()) as usize] = 3_();
    unimplemented!();
    unimplemented!();
    (*{
        let __idx = (unsafe { push_and_index_0(&mut v) }) as usize;
        &mut v[__idx]
    }) = 5_();
    unimplemented!();
    let mut p: *mut () = (&mut v[(2_()) as usize] as *mut ());
    (*p) = 9_();
    unimplemented!();
    unimplemented!();
    let mut h: Holder = <Holder>::default();
    unimplemented!();
    h.values[(1_()) as usize] = 6_();
    unimplemented!();
    h.points[(0_()) as usize].y = 5_();
    unimplemented!();
    unimplemented!();
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    (&mut (*hp)).values[(0_()) as usize] = (((&mut (*hp)).values[(1_()) as usize]) + (1_()));
    (&mut (*hp)).points[(0_()) as usize].x = (&mut (*hp)).values[(0_()) as usize];
    unimplemented!();
    let mut q: Point = h.points[(0_()) as usize];
    q.x = 0_();
    unimplemented!();
    let mut grid: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    grid[(1_()) as usize][(2_()) as usize] = 8_();
    unimplemented!();
    grid[(0_()) as usize][(1_()) as usize] = grid[(1_()) as usize][(2_()) as usize];
    grid[(1_()) as usize][(0_()) as usize] += grid[(0_()) as usize][(1_()) as usize];
    unimplemented!();
    let mut a: () = unimplemented!();
    a[(1_()) as usize] = ((a[(0_()) as usize]) + (a[(2_()) as usize]));
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
