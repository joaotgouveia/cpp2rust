extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut out: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (i < 10_()) {
        if ((i % 2_()) == unimplemented!()) {
            i.prefix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        i.prefix_inc();
    }
    let mut j: () = unimplemented!();
    'loop_: while (j < 10_()) {
        if ((i % 2_()) == unimplemented!()) {
            j.postfix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        j.postfix_inc();
    }
    let mut k1: () = unimplemented!();
    'loop_: while (k1 < 5_()) {
        let mut k2: () = unimplemented!();
        'loop_: while (k2 < 5_()) {
            let mut k3: () = unimplemented!();
            'loop_: while (k3 < 5_()) {
                if ((((k1 + k2) + k3) % 2_()) == unimplemented!()) {
                    k3.postfix_inc();
                    continue 'loop_;
                }
                out.prefix_inc();
                k3.postfix_inc();
            }
            if (((k1 + k2) % 2_()) == unimplemented!()) {
                k2.postfix_inc();
                continue 'loop_;
            }
            out.prefix_inc();
            k2.postfix_inc();
        }
        if ((k1 % 2_()) == unimplemented!()) {
            k1.postfix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        k1.postfix_inc();
    }
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
