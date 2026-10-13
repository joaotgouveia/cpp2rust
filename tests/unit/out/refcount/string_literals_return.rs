extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_greeting_0() -> Ptr<i8> {
    return Ptr::<i8>::from_string_literal(b"hello");
}
pub fn get_empty_1() -> Ptr<i8> {
    return Ptr::<i8>::from_string_literal(b"");
}
pub fn get_branch_2(mut x: i32) -> Ptr<i8> {
    if (x > 0) {
        return Ptr::<i8>::from_string_literal(b"positive");
    }
    return Ptr::<i8>::from_string_literal(b"non-positive");
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: Ptr<i8> = ({ get_greeting_0() });
    (if (((elem!(a, 0).read()) as i32) == (('h' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a[0] == \'h\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(a, 4).read()) as i32) == (('o' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a[4] == \'o\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(a, 5).read()) as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a[5] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut b: Ptr<i8> = ({ get_empty_1() });
    (if (((elem!(b, 0).read()) as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"b[0] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut c: Ptr<i8> = ({ get_branch_2(1) });
    (if (((elem!(c, 0).read()) as i32) == (('p' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"c[0] == \'p\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(c, 7).read()) as i32) == (('e' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"c[7] == \'e\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(c, 8).read()) as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"c[8] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut d: Ptr<i8> = ({ get_branch_2(-1_i32) });
    (if (((elem!(d, 0).read()) as i32) == (('n' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"d[0] == \'n\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(d, 11).read()) as i32) == (('e' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"d[11] == \'e\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((elem!(d, 12).read()) as i32) == (('\0' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"d[12] == \'\\0\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
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
