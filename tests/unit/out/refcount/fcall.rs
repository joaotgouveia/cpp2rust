extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f2_0(mut x: (), mut y: ()) -> () {
    return (x - y);
}
pub fn f3_1(mut x: (), mut y: (), mut z: ()) -> () {
    return (({ f2_0(x, y) }) + z);
}
pub fn f1_2(mut x: (), mut y: ()) -> () {
    let mut z1: () = ({ f2_0(x, y) });
    if (({ f2_0(z1, y) }) < unimplemented!()) {
        let mut z2: () = -({
            let _y: () = ({ f2_0(x, y) });
            let _z: () = y;
            f3_1(z1, _y, _z)
        });
        return ({
            f2_0(
                ({
                    let _x: () = z2;
                    let _y: () = ({ f3_1(z1, z2, x) });
                    f2_0(_x, _y)
                }),
                y,
            )
        });
    }
    return ({ f2_0(z1, x) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
