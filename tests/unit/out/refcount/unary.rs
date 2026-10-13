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
    let mut x: i32 = 0;
    let mut a: [i32; 3] = [0, 1, 2];
    'loop_: while (x < 3) {
        a[(x.postfix_inc()) as usize].prefix_inc();
    }
    let mut out: i32 = 0;
    'loop_: while (x != 0) {
        out += a[(x.prefix_dec()) as usize];
    }
    out.postfix_inc();
    let mut x2: i32 = out.prefix_dec();
    out.prefix_inc();
    let mut x3: i32 = out.postfix_dec();
    (if (((out.postfix_inc() + x2) + x3) == 19) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"out++ + x2 + x3 == 19"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut n: i32 = x2;
    let mut d: f64 = 1.5E+0;
    (if (n == x2) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"n == x2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (d == 1.5E+0) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"d == 1.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((n + n) == (2 * x2)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"+n + +n == 2 * x2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (a[(0) as usize] == a[(0) as usize]) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"+a[0] == a[0]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut wide: i128 = 5_i128;
    let mut w1: i128 = wide.postfix_inc();
    let mut w2: i128 = wide.prefix_inc();
    let mut w3: i128 = wide.postfix_dec();
    let mut w4: i128 = wide.prefix_dec();
    (if (((w1 == 5_i128) && (w2 == 7_i128)) && (w3 == 7_i128)) && (w4 == 5_i128) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"w1 == 5 && w2 == 7 && w3 == 7 && w4 == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut uwide: u128 = 0_u128;
    uwide.prefix_dec();
    uwide.postfix_inc();
    (if (uwide == 0_u128) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"uwide == 0"),
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
