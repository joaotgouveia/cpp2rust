extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn cmp_eq_0(mut rc: i32) -> i32 {
    return ((rc == -1_i32) as i32);
}
pub fn cmp_or_ptr_1(mut p: Ptr<i8>, mut q: Ptr<i8>) -> i32 {
    return (((!(p).is_null()) || (!(q).is_null())) as i32);
}
pub fn both_null_2(mut s1: Ptr<i8>, mut s2: Ptr<i8>) -> i32 {
    return ((((((s1).is_null()) as i32) != 0) && ((((s2).is_null()) as i32) != 0)) as i32);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0_i32;
    if ({
        b = a;
        b
    } != 0)
    {}
    'loop_: while (((({
        b = a;
        b
    }) != 0) as i32)
        != 0)
    {}
    if (a != 0) {}
    if (((a == b) as i32) != 0) {}
    if (((a < b) as i32) != 0) {}
    {
        &(4usize);
        {
            if (((a == b) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"a == b"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        26_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if !(({
                a = b;
                a
            }) != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"!(a = b)"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        27_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut c: bool = false;
    c = ({
        a = b;
        a
    } != 0);
    c = (((({
        b = a;
        b
    }) != 0) as i32)
        != 0);
    c = (a != 0);
    c = (((a == b) as i32) != 0);
    c = (((a < b) as i32) != 0);
    let mut x: i32 = 5;
    let mut y: i32 = 5;
    let mut eq: i32 = ((x == y) as i32);
    let mut lt: i32 = ((x < y) as i32);
    let mut neq: i32 = ((x != y) as i32);
    {
        &(4usize);
        {
            if (((eq == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"eq == 1"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        40_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((lt == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"lt == 0"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        41_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((neq == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"neq == 0"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        42_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut p1: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hi");
    let mut p2: Ptr<i8> = Ptr::<i8>::null();
    let mut either: i32 = (((!(p1).is_null()) || (!(p2).is_null())) as i32);
    let mut both: i32 = (((!(p1).is_null()) && (!(p2).is_null())) as i32);
    {
        &(4usize);
        {
            if (((either == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"either == 1"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        48_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((both == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"both == 0"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        49_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ cmp_eq_0(-1_i32) }) == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"cmp_eq(-1) == 1"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        51_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ cmp_eq_0(0) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"cmp_eq(0) == 0"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        52_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ cmp_or_ptr_1((p1).clone(), (p2).clone()) }) == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"cmp_or_ptr(p1, p2) == 1"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        53_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ cmp_or_ptr_1(Ptr::<i8>::null(), Ptr::<i8>::null()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"cmp_or_ptr(NULL, NULL) == 0"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        54_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ both_null_2(Ptr::<i8>::null(), Ptr::<i8>::null()) }) == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"both_null(NULL, NULL) == 1"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        55_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ both_null_2((p1).clone(), Ptr::<i8>::null()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"both_null(p1, NULL) == 0"),
                        Ptr::<i8>::from_string_literal(b"expr_as_bool_c.c"),
                        56_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_3(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
