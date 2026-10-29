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
pub struct S {
    pub x: (),
    pub y: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut base: () = 10_();
    let mut add_base: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let base: *mut () = &mut base;
        },
        |mut x: ()| -> () {
            return ((x) + (*base));
        }
    );
    unimplemented!();
    base = 100_();
    unimplemented!();
    let mut s: S = S { x: 1_(), y: 2_() };
    let mut sum: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let s: *mut S = &mut s;
        },
        || -> () {
            return (((*s).x) + ((*s).y));
        }
    );
    unimplemented!();
    s.x = 50_();
    unimplemented!();
    let mut counter: () = 0_();
    let mut bump: FnPtr<fn()> = lambda_unsafe!(
        {
            let counter: *mut () = &mut counter;
        },
        || {
            (*counter).postfix_inc();
        }
    );
    (unsafe { bump.call() });
    (unsafe { bump.call() });
    unimplemented!();
    let mut arr: [(); 4] = [3_(), 1_(), 2_(), 0_()];
    let mut swap: FnPtr<fn((), ())> = lambda_unsafe!(
        {
            let arr: *mut [(); 4] = &mut arr;
        },
        |mut i: (), mut j: ()| {
            let mut t: () = (*arr)[(j) as usize];
            (*arr)[(j) as usize] = (*arr)[(i) as usize];
            (*arr)[(i) as usize] = t;
        }
    );
    (unsafe { swap.call(0_(), 3_()) });
    unimplemented!();
    unimplemented!();
    let mut total: () = 0_();
    let mut add: FnPtr<fn(())> = lambda_unsafe!(
        {
            let t: *mut () = &mut total;
        },
        |mut x: ()| {
            (*t) += x;
        }
    );
    (unsafe { add.call(2_()) });
    (unsafe { add.call(3_()) });
    unimplemented!();
    let mut set_y: FnPtr<fn(())> = lambda_unsafe!(
        {
            let y: *mut () = &mut s.y;
        },
        |mut v: ()| {
            (*y) = v;
        }
    );
    (unsafe { set_y.call(9_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
