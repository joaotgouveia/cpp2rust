extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn matalloc_0(mut n: (), mut p: (), mut e: ()) -> () {
    let mut m: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        let mut j: () = 0_();
        'loop_: while (j < p) {
            unimplemented!().write(e);
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn matmul_1(mut m1: (), mut n1: (), mut p1: (), mut m2: (), mut n2: (), mut p2: ()) -> () {
    let mut m3: () = ({ matalloc_0(n1, p2, 0_()) });
    let mut i: () = 0_();
    'loop_: while (i < n1) {
        let mut j: () = 0_();
        let mut sum: () = 0_();
        'loop_: while (j < p2) {
            let mut k: () = 0_();
            'loop_: while (k < p1) {
                sum += ((unimplemented!().read()) * (unimplemented!().read()));
                k.prefix_inc();
            }
            unimplemented!().write(sum);
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut n: () = 1_();
    let mut p: () = 10_();
    let mut m1: () = ({ matalloc_0(n, p, 1_()) });
    let mut m2: () = ({ matalloc_0(p, n, 2_()) });
    let mut m3: () = ({
        let _m1: () = unimplemented!();
        let _n1: () = n;
        let _p1: () = p;
        let _m2: () = unimplemented!();
        let _n2: () = p;
        let _p2: () = n;
        matmul_1(_m1, _n1, _p1, _m2, _n2, _p2)
    });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
