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
    let mut arr: [i8; 7] = b"foobar\0".map(i8::from_byte);
    {
        &(4usize);
        {
            if ((((arr[(0) as usize] as i32) == ('f' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"arr[0] == \'f\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
                        6_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((arr[(3) as usize] as i32) == ('b' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"arr[3] == \'b\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
                        7_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((arr[(5) as usize] as i32) == ('r' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"arr[5] == \'r\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
                        8_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((arr[(6) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"arr[6] == \'\\0\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
                        9_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut split_pieces: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcdefghi");
    {
        &(4usize);
        {
            if (((((elem!(split_pieces, 0).read()) as i32) == ('a' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"split_pieces[0] == \'a\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
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
            if (((((elem!(split_pieces, 3).read()) as i32) == ('d' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"split_pieces[3] == \'d\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((elem!(split_pieces, 6).read()) as i32) == ('g' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"split_pieces[6] == \'g\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((elem!(split_pieces, 8).read()) as i32) == ('i' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"split_pieces[8] == \'i\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
                        17_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((elem!(split_pieces, 9).read()) as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"split_pieces[9] == \'\\0\'"),
                        Ptr::<i8>::from_string_literal(b"string_literals_concat_c.c"),
                        18_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
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
