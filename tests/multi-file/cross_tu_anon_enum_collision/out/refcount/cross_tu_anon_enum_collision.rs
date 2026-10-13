extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_ALPHA: anon_0 = 7;
pub fn a_value_1() -> i32 {
    let mut x: i32 = 0;
    x |= (anon_0_ALPHA as i32);
    return x;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ a_value_1() }) == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"a_value() == 7"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ b_value_3() }) == 9) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"b_value() == 9"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub type anon_4 = u32;
pub const anon_4_BETA: anon_4 = 9;
pub fn b_value_3() -> i32 {
    let mut x: i32 = 0;
    x |= (anon_4_BETA as i32);
    return x;
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
