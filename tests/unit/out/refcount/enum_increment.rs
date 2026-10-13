extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type color = u32;
pub const color_RED: color = 0;
pub const color_GREEN: color = 1;
pub const color_BLUE: color = 2;
pub const color_COLOR_LAST: color = 3;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut count: i32 = 0;
    let mut c: color = color_RED;
    'loop_: while ((((c as u32) < ((color_COLOR_LAST as i32) as u32)) as i32) != 0) {
        count.postfix_inc();
        c.postfix_inc();
    }
    {
        &(4usize);
        {
            if (((count == 3) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"count == 3"),
                        Ptr::<i8>::from_string_literal(b"enum_increment.c"),
                        10_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut c: color = color_RED;
    {
        &(4usize);
        {
            if ((((c.postfix_inc() as u32) == ((color_RED as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"c++ == RED"),
                        Ptr::<i8>::from_string_literal(b"enum_increment.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((c.prefix_inc() as u32) == ((color_BLUE as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"++c == BLUE"),
                        Ptr::<i8>::from_string_literal(b"enum_increment.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((c.postfix_dec() as u32) == ((color_BLUE as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"c-- == BLUE"),
                        Ptr::<i8>::from_string_literal(b"enum_increment.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((c.prefix_dec() as u32) == ((color_RED as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"--c == RED"),
                        Ptr::<i8>::from_string_literal(b"enum_increment.c"),
                        16_u32,
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
