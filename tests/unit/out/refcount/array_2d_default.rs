extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fill_row_0(mut row: Ptr<i8>, mut c: i8) {
    elem!(row, 0).write({ c });
    elem!(row, 1).write((('\0' as i32) as i8));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let grid: Value<Box<[Value<Box<[i8]>>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| Rc::new(RefCell::new((0..6).map(|_| 0_i8).collect::<Box<[i8]>>())))
            .collect::<Box<[Value<Box<[i8]>>]>>(),
    ));
    let mut i: i32 = 0;
    'loop_: while (((i < 3) as i32) != 0) {
        ({
            let _row: Ptr<i8> = (((grid.as_pointer() as Ptr<Value<Box<[i8]>>>)
                .offset(i)
                .read()
                .as_pointer()) as Ptr<i8>);
            let _c: i8 = ((('a' as i32) + i) as i8);
            fill_row_0(_row, _c)
        });
        i.postfix_inc();
    }
    {
        &(4usize);
        {
            if (((((*grid.borrow())[(0) as usize].borrow()[(0) as usize] as i32) == ('a' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"grid[0][0] == \'a\'"),
                        Ptr::<i8>::from_string_literal(b"array_2d_default.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((*grid.borrow())[(1) as usize].borrow()[(0) as usize] as i32) == ('b' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"grid[1][0] == \'b\'"),
                        Ptr::<i8>::from_string_literal(b"array_2d_default.c"),
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
            if (((((*grid.borrow())[(2) as usize].borrow()[(0) as usize] as i32) == ('c' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"grid[2][0] == \'c\'"),
                        Ptr::<i8>::from_string_literal(b"array_2d_default.c"),
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
            if (((((*grid.borrow())[(1) as usize].borrow()[(1) as usize] as i32) == ('\0' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"grid[1][1] == \'\\0\'"),
                        Ptr::<i8>::from_string_literal(b"array_2d_default.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    (*grid.borrow())[(2) as usize].borrow_mut()[(5) as usize] = (('z' as i32) as i8);
    {
        &(4usize);
        {
            if (((((*grid.borrow())[(2) as usize].borrow()[(5) as usize] as i32) == ('z' as i32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"grid[2][5] == \'z\'"),
                        Ptr::<i8>::from_string_literal(b"array_2d_default.c"),
                        18_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
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
