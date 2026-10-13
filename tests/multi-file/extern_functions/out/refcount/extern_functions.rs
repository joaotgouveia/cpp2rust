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
    {
        &(4usize);
        {
            if (((({ helper_0(42) }) == 43) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"helper(42) == 43"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        6_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn unrelated1_2() -> i32 {
    return 1;
}
pub fn unrelated2_3() -> i32 {
    return 2;
}
pub fn unrelated3_4() -> i32 {
    return 3;
}
pub fn helper_0(mut x: i32) -> i32 {
    &({ unrelated1_2() });
    &({ unrelated2_3() });
    &({ unrelated3_4() });
    return (x + 1);
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
