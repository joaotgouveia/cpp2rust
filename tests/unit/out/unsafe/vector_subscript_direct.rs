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
    pub values: std_vector_int__std_allocator_int__,
    pub points: std_vector_Point__std_allocator_Point__,
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            values: (*_a0).values,
            points: (*_a0).points,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.values = ((*_a0).values).clone();
        self.points = ((*_a0).points).clone();
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn push_and_index_0(v: *mut std_vector_int__std_allocator_int__) -> () {
    (unsafe {
        let mut ___x: () = 42_();
        (*v).push_back_58(&mut ___x)
    });
    return ((unsafe { (*v).size() }) - (1_()));
}
pub unsafe fn sum_ref_1(v: *const std_vector_int__std_allocator_int__) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (unsafe { (*v).size() })) {
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
    let mut v: std_vector_int__std_allocator_int__ =
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
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
    (unsafe { h.values.resize_40(2_()) });
    h.values[(1_()) as usize] = 6_();
    (unsafe {
        let mut ___x: Point = Point { x: 1_(), y: 2_() };
        h.points.push_back_58(&mut ___x)
    });
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
    let mut grid : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = <std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ >::default() ;
    (unsafe {
        let mut ___x: std_vector_int__std_allocator_int__ = {
            let mut __tmp_0: () = 0_();
            std_vector_int__std_allocator_int__::new_2({ 3_() }, { &mut __tmp_0 }, None)
        };
        grid.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: std_vector_int__std_allocator_int__ = {
            let mut __tmp_1: () = 0_();
            std_vector_int__std_allocator_int__::new_2({ 3_() }, { &mut __tmp_1 }, None)
        };
        grid.push_back_58(&mut ___x)
    });
    grid[(1_()) as usize][(2_()) as usize] = 8_();
    unimplemented!();
    grid[(0_()) as usize][(1_()) as usize] = grid[(1_()) as usize][(2_()) as usize];
    grid[(1_()) as usize][(0_()) as usize] += grid[(0_()) as usize][(1_()) as usize];
    unimplemented!();
    let mut a: std_array_int__3_ = unimplemented!();
    a[(1_()) as usize] = ((a[(0_()) as usize]) + (a[(2_()) as usize]));
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
