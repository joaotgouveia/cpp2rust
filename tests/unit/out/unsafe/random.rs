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
pub struct Pair {
    pub x: (),
    pub y: (),
    pub a: [(); 5],
    pub r: *mut (),
    pub p: *mut (),
    pub pair: *mut Pair,
    pub ap: [*mut (); 2],
}
impl Pair {
    pub unsafe fn method(&mut self) {
        self.x.postfix_inc();
        self.y.prefix_inc();
        self.a[(4_()) as usize] = 1_();
        (*self.r) = 1_();
        self.p = std::ptr::null_mut();
        self.pair = std::ptr::null_mut();
        self.ap[(unimplemented!()) as usize] = std::ptr::null_mut();
    }
    pub unsafe fn as_val(&mut self) -> () {
        return self.x;
    }
    pub unsafe fn as_ref(&mut self) -> *mut () {
        return &mut self.x;
    }
    pub unsafe fn as_ptr(&mut self) -> *mut () {
        return (&mut self.x as *mut ());
    }
}
pub unsafe fn zero_0() -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X1 {}
pub unsafe fn foo_1(mut x1: (), x2: *mut (), mut x3: *mut (), p2: *mut Pair, mut p3: *mut Pair) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x1: () = 1_();
    let mut c1: () = x1;
    let rx1: *mut () = &mut x1;
    let mut px1: *mut () = (&mut x1 as *mut ());
    let mut x2: () = (*rx1);
    let rx2: *mut () = rx1;
    let mut px2: *mut () = (rx1);
    let mut x3: () = (*px1);
    let rx3: *mut () = &mut (*px1);
    let mut px3: *mut () = px1;
    let mut res: () = ((x1) + (x2));
    res = ((x1) + (x2));
    let mut y1: Pair = Pair {
        x: 1_(),
        y: 2_(),
        a: [1_(), 2_(), 3_(), 4_(), 5_()],
        r: &mut x1,
        p: std::ptr::null_mut(),
        pair: std::ptr::null_mut(),
        ap: [std::ptr::null_mut(), std::ptr::null_mut()],
    };
    let mut y4: Pair = Pair {
        x: y1.x,
        y: y1.y,
        a: [
            y1.a[(unimplemented!()) as usize],
            y1.a[(1_()) as usize],
            y1.a[(2_()) as usize],
            y1.a[(3_()) as usize],
            y1.a[(4_()) as usize],
        ],
        r: y1.r,
        p: y1.p,
        pair: y1.pair,
        ap: [y1.ap[(unimplemented!()) as usize], y1.ap[(1_()) as usize]],
    };
    let ry1: *mut Pair = &mut y1;
    let mut py1: *mut Pair = (&mut y1 as *mut Pair);
    let mut y2: Pair = Pair {
        x: (*ry1).x,
        y: (*ry1).y,
        a: [
            (*ry1).a[(unimplemented!()) as usize],
            (*ry1).a[(1_()) as usize],
            (*ry1).a[(2_()) as usize],
            (*ry1).a[(3_()) as usize],
            (*ry1).a[(4_()) as usize],
        ],
        r: (*ry1).r,
        p: (*ry1).p,
        pair: (*ry1).pair,
        ap: [
            (*ry1).ap[(unimplemented!()) as usize],
            (*ry1).ap[(1_()) as usize],
        ],
    };
    let ry2: *mut Pair = ry1;
    let mut py2: *mut Pair = (ry1);
    let mut y3: Pair = Pair {
        x: (*py1).x,
        y: (*py1).y,
        a: [
            (*py1).a[(unimplemented!()) as usize],
            (*py1).a[(1_()) as usize],
            (*py1).a[(2_()) as usize],
            (*py1).a[(3_()) as usize],
            (*py1).a[(4_()) as usize],
        ],
        r: (*py1).r,
        p: (*py1).p,
        pair: (*py1).pair,
        ap: [
            (*py1).ap[(unimplemented!()) as usize],
            (*py1).ap[(1_()) as usize],
        ],
    };
    let ry3: *mut Pair = &mut (*py1);
    let mut py3: *mut Pair = py1;
    py3 = std::ptr::null_mut();
    let mut ptr2pair: *mut Pair = py3;
    (unsafe {
        let _x1: () = x1;
        let _x2: *mut () = &mut x1;
        let _x3: *mut () = (&mut x1 as *mut ());
        let _p2: *mut Pair = &mut y1;
        let _p3: *mut Pair = (&mut y1 as *mut Pair);
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    (unsafe {
        let _x1: () = (*rx1);
        let _x2: *mut () = rx1;
        let _x3: *mut () = (rx1);
        let _p2: *mut Pair = ry1;
        let _p3: *mut Pair = (ry1);
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    (unsafe {
        let _x1: () = (*px1);
        let _x2: *mut () = &mut (*px1);
        let _x3: *mut () = px1;
        let _p2: *mut Pair = &mut (*py1);
        let _p3: *mut Pair = py1;
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    let cr1: *mut () = &mut c1;
    let mut cp1: *mut () = (&mut c1 as *mut ());
    x1 = c1;
    x1 = 1_();
    x1 = (*cr1);
    x1 = (*cp1);
    (*rx1) = c1;
    (*rx2) = (*cr1);
    (*rx3) = (*cp1);
    (*px1) = c1;
    (*px2) = (*cr1);
    (*px3) = (*cp1);
    px1 = (&mut c1 as *mut ());
    px2 = (cr1);
    px3 = cp1;
    y1.x = 2_();
    y1.y = 3_();
    y1.a[(unimplemented!()) as usize] = 100_();
    (*y1.r) = 10_();
    y1.p = px3;
    px3 = px2;
    y1.pair = (&mut y3 as *mut Pair);
    (*y1.pair).x = 100_();
    (*y1.pair).pair = (&mut y2 as *mut Pair);
    (*(*y1.pair).pair).x = 100_();
    y1.ap[(unimplemented!()) as usize] = (&mut x1 as *mut ());
    y1.ap[(1_()) as usize] = (&mut x2 as *mut ());
    (*y1.ap[(unimplemented!()) as usize]) = unimplemented!();
    c1 = ((x1) + (1_()));
    let mut j: () = unimplemented!();
    let mut new_y: Pair = Pair {
        x: 1_(),
        y: 2_(),
        a: [1_(), 2_(), 3_(), 4_(), 5_()],
        r: &mut j,
        p: std::ptr::null_mut(),
        pair: std::ptr::null_mut(),
        ap: [std::ptr::null_mut(), std::ptr::null_mut()],
    };
    y1.x = new_y.x;
    let mut i: () = 1_();
    y1.a[(i) as usize] = -1_();
    x1.postfix_inc();
    x1.prefix_inc();
    y1.x.postfix_inc();
    (*y1.pair).pair = (&mut y2 as *mut Pair);
    (*(*y1.pair).pair).x = 10_();
    (unsafe { Pair::method(&mut y1) });
    y1.pair = (&mut y2 as *mut Pair);
    y2.pair = (&mut y3 as *mut Pair);
    (unsafe { Pair::method(&mut (*(*y1.pair).pair)) });
    let mut x: X1 = <X1>::default();
    let mut y: X1 = <X1>::default();
    x1 = ((unsafe { zero_0() }) + (y1.x));
    y1.x = ((unsafe { zero_0() }) + (5_()));
    let mut ptr2ptr_1: *mut *mut () = (&mut px1 as *mut *mut ());
    let mut ptr2ptr_2: *mut *mut Pair = (&mut py1 as *mut *mut Pair);
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
