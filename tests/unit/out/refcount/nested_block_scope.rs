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
    let mut x: i32 = 1;
    {
        let mut x: i32 = 2;
        (if (x == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"x == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        {
            let mut x: i32 = 3;
            (if (x == 3) {
                &(0);
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"x == 3"),
                        Ptr::<i8>::from_string_literal(file!().as_bytes()),
                        line!() as u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            });
        }
        (if (x == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"x == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    (if (x == 1) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"x == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        let mut y: i32 = i;
        {
            let mut y: i32 = 10;
            sum += y;
        }
        sum += y;
        i.postfix_inc();
    }
    (if (sum == 33) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"sum == 33"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    if (x == 1) {
        let mut x: i32 = 5;
        {
            let mut x: i32 = 6;
            (if (x == 6) {
                &(0);
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"x == 6"),
                        Ptr::<i8>::from_string_literal(file!().as_bytes()),
                        line!() as u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            });
        }
        (if (x == 5) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"x == 5"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    (if (x == 1) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"x == 1"),
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
