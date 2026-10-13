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
    let mut flag: u32 = 7_u32;
    let mut b1: bool = (flag != 0);
    let mut b2: bool = (0_u32 != 0);
    {
        &(4usize);
        {
            if b1 {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"b1"),
                        Ptr::<i8>::from_string_literal(b"int_to_bool_explicit_c.c"),
                        8_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if !(b2) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"!b2"),
                        Ptr::<i8>::from_string_literal(b"int_to_bool_explicit_c.c"),
                        9_u32,
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
