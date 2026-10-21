extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Elem {
    pub id: (),
    pub hist: [(); 4],
    pub coeffs: (),
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct State {
    pub count: (),
    pub grid: [[(); 3]; 2],
    pub arr: (),
    pub elems: (),
    pub rows: (),
}
impl Default for State {
    fn default() -> Self {
        unsafe {
            State {
                count: unimplemented!(),
                grid: [[unimplemented!(); 3]; 2],
                arr: unimplemented!(),
                elems: unimplemented!(),
                rows: unimplemented!(),
            }
        }
    }
}
pub unsafe fn sum_0(mut p: *const (), mut n: ()) -> () {
    let mut s: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (n)) {
        s += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn fill_1(mut s: *mut State) {
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (2_())) {
        let mut j: () = unimplemented!();
        'loop_: while ((j) < (3_())) {
            (*s).grid[(i) as usize][(j) as usize] = (((i) * (3_())) + (j));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    (*unimplemented!()) = 7_();
    unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (3_())) {
        (*unimplemented!()).id = i;
        (*unimplemented!()).hist[(i) as usize] = ((10_()) * (i));
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: State = <State>::default();
    (unsafe { fill_1((&mut s as *mut State)) });
    unimplemented!();
    unimplemented!();
    let mut e: *mut Elem = (&mut (*unimplemented!()) as *mut Elem);
    let mut h: *mut () = (*e).hist.as_mut_ptr();
    let mut c: *mut () = (&mut (*unimplemented!()) as *mut ());
    let mut id: *mut () = (&mut (*unimplemented!()).id as *mut ());
    let mut k: () = unimplemented!();
    'loop_: while ((k) < (4_())) {
        (*h.offset((k) as isize)) += k;
        k.prefix_inc();
    }
    (*c) += 5_();
    (*id) = 11_();
    s.count = 3_();
    unimplemented!();
    unimplemented!();
    let mut row: *mut () = (&mut (*unimplemented!()) as *mut ());
    (*row.offset((2_()) as isize)) = 9_();
    unimplemented!();
    let mut t: State = s.clone();
    (*unimplemented!()).hist[(unimplemented!()) as usize] = 100_();
    t.grid[(unimplemented!()) as usize][(unimplemented!()) as usize] = 100_();
    (*unimplemented!()) = 100_();
    unimplemented!();
    unimplemented!();
    let mut p: *mut State = (&mut t as *mut State);
    (*unimplemented!()).hist[(3_()) as usize] =
        (((*p).grid[(1_()) as usize][(1_()) as usize]) + (*unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
