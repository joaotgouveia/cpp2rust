extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn len_0(s: *const [(); 5]) -> () {
    let mut n: () = 0_();
    'loop_: while (((*s)[(n) as usize]) != ('\0' as ())) {
        n.prefix_inc();
    }
    return n;
}
pub unsafe fn len5_1(mut s: *const ()) -> () {
    let mut n: () = 0_();
    'loop_: while ((*s.offset((n) as isize)) != ('\0' as ())) {
        n.prefix_inc();
    }
    return n;
}
pub unsafe fn sum_2(a: *const [(); 3]) -> () {
    return ((((*a)[(0_()) as usize]) + ((*a)[(1_()) as usize])) + ((*a)[(2_()) as usize]));
}
pub unsafe fn fill_3(a: *mut [(); 3], mut v: ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (3_())) {
        (*a)[(i) as usize] = v;
        i.prefix_inc();
    }
}
pub unsafe fn sum_twice_4(a: *const [(); 3]) -> () {
    return ((unsafe { sum_2(a) }) + (unsafe { sum_2(a) }));
}
pub unsafe fn sum_ptr_5(mut p: *const ()) -> () {
    return (((*p.offset((0_()) as isize)) + (*p.offset((1_()) as isize)))
        + (*p.offset((2_()) as isize)));
}
pub unsafe fn sum_decayed_6(a: *const [(); 3]) -> () {
    return (unsafe { sum_ptr_5((*a).as_ptr()) });
}
pub unsafe fn bump_ptr_7(mut p: *mut ()) {
    (*p.offset((0_()) as isize)) += 1_();
}
pub unsafe fn bump_decayed_8(a: *mut [(); 3]) {
    (unsafe { bump_ptr_7((*a).as_mut_ptr()) });
}
pub unsafe fn fill_and_sum_9(a: *mut [(); 3], mut v: (), out: *mut ()) {
    (unsafe {
        let _a: *mut [(); 3] = a;
        let _v: () = v;
        fill_3(_a, _v)
    });
    (*out) = (unsafe { sum_twice_4(a) });
}
pub unsafe fn pick_10(s: *const [(); 5]) -> *const [(); 5] {
    return s;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: (),
    pub y: (),
}
pub unsafe fn sum_points_11(p: *const [Point; 2]) -> () {
    return (((((*p)[(0_()) as usize].x) + ((*p)[(0_()) as usize].y)) + ((*p)[(1_()) as usize].x))
        + ((*p)[(1_()) as usize].y));
}
pub unsafe fn shift_points_12(p: *mut [Point; 2], mut d: ()) {
    (*p)[(0_()) as usize].x += d;
    (*p)[(1_()) as usize].y += d;
}
pub unsafe fn total_len_13(names: *mut [*const (); 2]) -> () {
    return ((unsafe { len5_1((*names)[(0_()) as usize]) })
        + (unsafe { len5_1((*names)[(1_()) as usize]) }));
}
pub static mut kTable_14: std::cell::LazyCell<[[(); 3]; 2]> =
    std::cell::LazyCell::new(|| unsafe { [[1_(), 2_(), 3_()], [4_(), 5_(), 6_()]] });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    let mut buf: [(); 5] = std::mem::transmute(*b"abcd\0");
    unimplemented!();
    let mut arr: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    unimplemented!();
    (unsafe { fill_3(&mut arr, 7_()) });
    unimplemented!();
    unimplemented!();
    let mut out: () = 0_();
    (unsafe { fill_and_sum_9(&mut arr, 2_(), &mut out) });
    unimplemented!();
    unimplemented!();
    let lit: *const [(); 5] = &std::mem::transmute(*b"beta\0");
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut pts: [Point; 2] = [Point { x: 1_(), y: 2_() }, Point { x: 3_(), y: 4_() }];
    unimplemented!();
    (unsafe { shift_points_12(&mut pts, 10_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { bump_decayed_8(&mut arr) });
    unimplemented!();
    let mut names: [*const (); 2] = [c"ab".as_ptr(), c"cde".as_ptr()];
    unimplemented!();
    let mut rows: [[(); 3]; 2] = [[1_(), 2_(), 3_()], [4_(), 5_(), 6_()]];
    let mut p: *mut [(); 3] = rows.as_mut_ptr();
    p.prefix_inc();
    unimplemented!();
    let r: *mut [(); 3] = &mut (*p);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut q: *mut [(); 3] = rows.as_mut_ptr();
    unimplemented!();
    unimplemented!();
    (*q.offset((1_()) as isize))[(2_()) as usize] = 8_();
    unimplemented!();
    rows[(0_()) as usize][(1_()) as usize] = rows[(0_()) as usize][(2_()) as usize];
    (*q.offset((1_()) as isize))[(0_()) as usize] = (*q.offset((0_()) as isize))[(1_()) as usize];
    (*q.offset((1_()) as isize))[(1_()) as usize] += (*q.offset((0_()) as isize))[(2_()) as usize];
    unimplemented!();
    unimplemented!();
    let mut key: [(); 4] = [0_(), 4_(), 5_(), 6_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const kTable_14);
}
