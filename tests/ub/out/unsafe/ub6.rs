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
    pub x1: *mut (),
    pub x2: *mut (),
}
pub unsafe fn mkPair_0(x1: *mut (), x2: *mut ()) -> Pair {
    return Pair { x1: x1, x2: x2 };
}
pub unsafe fn fill_1(arr: *mut (), n1: *mut ()) {
    let mut n2: () = (*n1);
    let mut pair: Pair = (unsafe {
        let _x1: *mut () = n1;
        let _x2: *mut () = &mut n2;
        mkPair_0(_x1, _x2)
    });
    (*unimplemented!()) = (pair.x1);
    (*unimplemented!()) = (pair.x2);
}
pub unsafe fn any_2(arr: *mut (), n1: *mut ()) -> () {
    let mut out: () = false;
    let mut i: () = 0_();
    'loop_: while ((i) < (*n1)) {
        out = (out) || ((*(*unimplemented!())) == (0_()));
        i.prefix_inc();
    }
    return out;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut n: () = 2_();
    let mut arr: () = (unsafe { make_unique_3(n) });
    (unsafe { fill_1(&mut arr, &mut n) });
    return (unsafe { any_2(&mut arr, &mut n) });
}
pub unsafe fn __cpp2rust_init_globals() {}
