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
    let mut x1: i32 = -1_i32;
    {
        &(4usize);
        {
            if (((x1 == -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"x1 == -1"),
                        Ptr::<i8>::from_string_literal(b"signed_integer.c"),
                        5_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut x2: i8 = -1_i8;
    {
        &(4usize);
        {
            if ((((x2 as i32) == -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"x2 == -1"),
                        Ptr::<i8>::from_string_literal(b"signed_integer.c"),
                        7_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut u1: u32 = 5_u32;
    let mut u2: u32 = (u1).wrapping_neg();
    {
        &(4usize);
        {
            if (((u2 == 4294967291_u32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"u2 == 4294967291u"),
                        Ptr::<i8>::from_string_literal(b"signed_integer.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut c1: i8 = -1_i8;
    {
        &(4usize);
        {
            if (((((c1 as u8) as i32) == 255) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"(unsigned char)c1 == 0xFF"),
                        Ptr::<i8>::from_string_literal(b"signed_integer.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
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
