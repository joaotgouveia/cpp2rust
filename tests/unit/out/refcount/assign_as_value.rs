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
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..2).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut p: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
    let mut q: Ptr<i8> = Ptr::<i8>::null();
    q = {
        p += 1;
        (p).clone()
    };
    {
        &(4usize);
        {
            if ((({ (q).clone() } == { (buf.as_pointer() as Ptr<i8>).offset((1) as isize) })
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"q == buf + 1"),
                        Ptr::<i8>::from_string_literal(b"assign_as_value.c"),
                        7_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut out: i8 = 0_i8;
    'switch: {
        match {
            (({
                out = (('x' as i32) as i8);
                out
            }) as i32)
        } {
            __v if __v == ('x' as i32) => {
                {
                    &(4usize);
                    { {} }
                };
                break 'switch;
            }
            _ => {
                {
                    &(4usize);
                    {
                        {
                            ({
                                __assert_fail_0(
                                    Ptr::<i8>::from_string_literal(b"0"),
                                    Ptr::<i8>::from_string_literal(b"assign_as_value.c"),
                                    15_u32,
                                    Ptr::<i8>::from_string_literal(b"int main(void)"),
                                )
                            });
                        }
                    }
                };
                break 'switch;
            }
        }
    };
    {
        &(4usize);
        {
            if ((((out as i32) == ('x' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"out == \'x\'"),
                        Ptr::<i8>::from_string_literal(b"assign_as_value.c"),
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
