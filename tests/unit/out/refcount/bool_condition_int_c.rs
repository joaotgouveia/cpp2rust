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
    let mut n: i32 = 3;
    let mut zero: i32 = 0;
    let mut u: u32 = 4_u32;
    let mut ul: u64 = 5_u64;
    let mut ll: i64 = 6_i64;
    let mut ch: i8 = (('a' as i32) as i8);
    if (n != 0) {
        {
            &(4usize);
            { {} }
        };
    }
    if !(n != 0) {
        {
            &(4usize);
            {
                {
                    ({
                        __assert_fail_0(
                            Ptr::<i8>::from_string_literal(b"false"),
                            Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                            16_u32,
                            Ptr::<i8>::from_string_literal(b"int main()"),
                        )
                    });
                }
            }
        };
    }
    if (zero != 0) {
        {
            &(4usize);
            {
                {
                    ({
                        __assert_fail_0(
                            Ptr::<i8>::from_string_literal(b"false"),
                            Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                            19_u32,
                            Ptr::<i8>::from_string_literal(b"int main()"),
                        )
                    });
                }
            }
        };
    }
    if !(zero != 0) {
        {
            &(4usize);
            { {} }
        };
    }
    if (u != 0) {
        {
            &(4usize);
            { {} }
        };
    }
    if (ul != 0) {
        {
            &(4usize);
            { {} }
        };
    }
    if (ll != 0) {
        {
            &(4usize);
            { {} }
        };
    }
    if (ch != 0) {
        {
            &(4usize);
            { {} }
        };
    }
    let mut loop_count: i32 = 0;
    let mut counter: i32 = 3;
    'loop_: while (counter != 0) {
        counter.prefix_dec();
        loop_count.prefix_inc();
    }
    {
        &(4usize);
        {
            if (((loop_count == 3) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"loop_count == 3"),
                        Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                        44_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut i: i32 = 5;
    'loop_: while (i != 0) {
        loop_count.prefix_inc();
        i.prefix_dec();
    }
    {
        &(4usize);
        {
            if (((loop_count == 8) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"loop_count == 8"),
                        Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                        49_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut t: i32 = if (n != 0) { 100 } else { 200 };
    {
        &(4usize);
        {
            if (((t == 100) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"t == 100"),
                        Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                        52_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut t2: i32 = if (zero != 0) { 100 } else { 200 };
    {
        &(4usize);
        {
            if (((t2 == 200) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"t2 == 200"),
                        Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                        54_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut t7: i32 = (!(n != 0) as i32);
    {
        &(4usize);
        {
            if (((t7 == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"t7 == 0"),
                        Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                        56_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut t8: i32 = (!(zero != 0) as i32);
    {
        &(4usize);
        {
            if (((t8 == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"t8 == 1"),
                        Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                        58_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut b1: bool = (n != 0);
    {
        &(4usize);
        {
            if b1 {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"b1"),
                        Ptr::<i8>::from_string_literal(b"bool_condition_int_c.c"),
                        61_u32,
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
