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
fn main_0() -> i32 {
    let mut out: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        if ((i % 2) == 0) {
            i.prefix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        i.prefix_inc();
    }
    let mut j: i32 = 0;
    'loop_: while (j < 10) {
        if ((i % 2) == 0) {
            j.postfix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        j.postfix_inc();
    }
    let mut k1: i32 = 0;
    'loop_: while (k1 < 5) {
        let mut k2: i32 = 0;
        'loop_: while (k2 < 5) {
            let mut k3: i32 = 0;
            'loop_: while (k3 < 5) {
                if ((((k1 + k2) + k3) % 2) == 0) {
                    k3.postfix_inc();
                    continue 'loop_;
                }
                out.prefix_inc();
                k3.postfix_inc();
            }
            if (((k1 + k2) % 2) == 0) {
                k2.postfix_inc();
                continue 'loop_;
            }
            out.prefix_inc();
            k2.postfix_inc();
        }
        if ((k1 % 2) == 0) {
            k1.postfix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        k1.postfix_inc();
    }
    (if (out == 81) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"out == 81"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_0(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
