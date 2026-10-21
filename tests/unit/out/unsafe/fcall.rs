extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f2_0(mut x: (), mut y: ()) -> () {
    return ((x) - (y));
}
pub unsafe fn f3_1(mut x: (), mut y: (), mut z: ()) -> () {
    return ((unsafe { f2_0(x, y) }) + (z));
}
pub unsafe fn f1_2(mut x: (), mut y: ()) -> () {
    let mut z1: () = (unsafe { f2_0(x, y) });
    if ((unsafe { f2_0(z1, y) }) < (unimplemented!())) {
        let mut z2: () = -(unsafe {
            let _y: () = (unsafe { f2_0(x, y) });
            let _z: () = y;
            f3_1(z1, _y, _z)
        });
        return (unsafe {
            f2_0(
                (unsafe {
                    let _x: () = z2;
                    let _y: () = (unsafe { f3_1(z1, z2, x) });
                    f2_0(_x, _y)
                }),
                y,
            )
        });
    }
    return (unsafe { f2_0(z1, x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
