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
pub struct In {
    pub a: (),
    pub b: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub x: (),
    pub in_: In,
    pub z: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut src: S = S {
        x: 1_(),
        in_: In { a: 2_(), b: 3_() },
        z: 4_(),
    };
    let mut p: *mut S = (unimplemented!() as *mut S);
    unimplemented!();
    (unsafe {
        let ___dest: () = ((&mut (*p).x as *mut ()) as ());
        let ___n: () = ::std::mem::size_of::<S>();
        memcpy_0(___dest, ((&mut src as *mut S) as ()), ___n)
    });
    unimplemented!();
    let mut n: In = In { a: 5_(), b: 6_() };
    (unsafe {
        let ___src: () = ((&mut n as *mut In) as ());
        let ___n: () = ::std::mem::size_of::<In>();
        memcpy_0(((&mut (*p).in_ as *mut In) as ()), ___src, ___n)
    });
    unimplemented!();
    let mut bz: *mut () = ((&mut (*p).z as *mut ()) as *mut ());
    let mut i: () = 0_();
    'loop_: while ((((i) < (4_())) as ()) != 0) {
        (*bz.offset((i) as isize)) = 1_();
        i.postfix_inc();
    }
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
