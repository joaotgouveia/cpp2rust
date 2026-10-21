extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn convert_without_rhs_0() {
    let mut x: () = 0_();
    let mut y: () = 1_();
    x = 0_();
    x = ((y) + (1_()));
    y = 0_();
    y = ((x) + (1_()));
    x += 1_();
    y += 1_();
    y = 0_();
    x = 0_();
    let mut z: () = ((x) + (y));
    z = (((x) + (y)) + (1_()));
    let mut arr: [(); 2] = [1_(), 2_()];
    let mut w: () = ((arr[(y) as usize]) + (arr[(x) as usize]));
    w += (((z) + (y)) + (x));
    let mut arr2: [(); 3] = [('a' as ()), ('b' as ()), ('c' as ())];
    let mut p1: *const () = (&mut x as *mut ()).cast_const();
    let mut c: () = arr2[(*p1) as usize];
    c = arr2[(*p1) as usize];
    let mut p2: *mut () = (&mut x as *mut ());
    (*p2) = 1_();
    let r: *mut () = &mut x;
    (*r) = 1_();
}
pub unsafe fn convert_with_rhs_1() {
    let mut x: () = 0_();
    x = ((x) + (1_()));
    let mut y: () = 0_();
    y = ((y) + (1_()));
    let mut arr: [(); 2] = [1_(), 2_()];
    arr[(y) as usize] = ((y) + (1_()));
    arr[(x) as usize] = ((x) + (1_()));
    arr[(x) as usize] = ((arr[(y) as usize]) + (1_()));
    let z: *mut () = &mut x;
    x += (*z);
    y += (*z);
    let mut p: *mut () = (&mut x as *mut ());
    x += (*p);
    y += (*p);
    p = (&mut arr[(0_()) as usize] as *mut ());
    arr[(0_()) as usize] = (*p);
    (*z) += x;
    (*z) += y;
    (*z) += (*p);
    (*p) += ((y) + (x));
    (*p) += ((x) + (*z));
    (*p) += ((y) + (*z));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { convert_without_rhs_0() });
    (unsafe { convert_with_rhs_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
