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
    let mut out: () = 0_();
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if ((i % 2_()) == 0_()) {
            i.prefix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        i.prefix_inc();
    }
    let mut j: () = 0_();
    'loop_: while (j < 10_()) {
        if ((i % 2_()) == 0_()) {
            j.postfix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        j.postfix_inc();
    }
    let mut k1: () = 0_();
    'loop_: while (k1 < 5_()) {
        let mut k2: () = 0_();
        'loop_: while (k2 < 5_()) {
            let mut k3: () = 0_();
            'loop_: while (k3 < 5_()) {
                if ((((k1 + k2) + k3) % 2_()) == 0_()) {
                    k3.postfix_inc();
                    continue 'loop_;
                }
                out.prefix_inc();
                k3.postfix_inc();
            }
            if (((k1 + k2) % 2_()) == 0_()) {
                k2.postfix_inc();
                continue 'loop_;
            }
            out.prefix_inc();
            k2.postfix_inc();
        }
        if ((k1 % 2_()) == 0_()) {
            k1.postfix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        k1.postfix_inc();
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
