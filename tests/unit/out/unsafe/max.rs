extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut calls_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut xs_1: std::cell::LazyCell<[(); 2]> =
    std::cell::LazyCell::new(|| unsafe { [1_(), 2_()] });
pub unsafe fn pick_2(mut i: ()) -> *mut () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)).prefix_inc();
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut xs_1))[(i) as usize];
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Bag {
    pub n: (),
}
impl Bag {
    pub unsafe fn new(mut n: ()) -> Self {
        let mut this = Self { n: n };
        this
    }
    pub unsafe fn copy_from(o: *const Bag) -> Self {
        let mut this = Self { n: (*o).n };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Bag) -> *mut Bag {
        self.n = (*_a0).n;
        return &mut (*(self as *mut Bag));
    }
}
impl std::cmp::Ord for Bag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_3(self as *const Bag, other as *const Bag) {
                std::cmp::Ordering::Less
            } else if operator_lt_3(other as *const Bag, self as *const Bag) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_3(self as *const Bag, other as *const Bag))
                && !(operator_lt_3(other as *const Bag, self as *const Bag))
        }
    }
}
impl std::cmp::Eq for Bag {}
impl Clone for Bag {
    fn clone(&self) -> Self {
        unsafe { Bag::copy_from(self as *const Bag) }
    }
}
pub unsafe fn operator_lt_3(a: *const Bag, b: *const Bag) -> () {
    return (((*a).n) < ((*b).n));
}
pub unsafe fn make_4(mut n: ()) -> Bag {
    return Bag::new({ n });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x1: () = 1_();
    let mut x2: () = 2_();
    let mut x3: () = 10_();
    let mut x4: () = 20_();
    let mut p1: *mut () = (&mut x1 as *mut ());
    let mut p2: *mut () = (&mut x2 as *mut ());
    let mut r1: () = (*unimplemented!());
    let mut r2: () = (*unimplemented!());
    let mut r3: () = (*unimplemented!());
    let mut r4: () = (*unimplemented!());
    let mut r5: () = (*unimplemented!());
    unimplemented!();
    let mut values: [(); 4] = [1_(), 7_(), 7_(), 3_()];
    let mut max: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) = 0_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const calls_0);
    std::cell::LazyCell::force(&*&raw const xs_1);
}
