extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(mut x: i32) -> i32 {
    return (x + 1);
}
pub fn dec_1(mut x: i32) -> i32 {
    return (x - 1);
}
pub fn identity_2(mut x: i32) -> i32 {
    return x;
}
pub fn pick_3(mut mode: i32) -> FnPtr<fn(i32) -> i32> {
    return if (mode > 0) {
        FnPtr::<fn(i32) -> i32>::new(inc_0)
    } else {
        if (mode < 0) {
            FnPtr::<fn(i32) -> i32>::new(dec_1)
        } else {
            FnPtr::<fn(i32) -> i32>::new(identity_2)
        }
    };
}
pub fn apply_4(mut fn_: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    let mut actual: FnPtr<fn(i32) -> i32> = if !(fn_).is_null() {
        (fn_).clone()
    } else {
        FnPtr::<fn(i32) -> i32>::new(identity_2)
    };
    return ({ actual.call(x) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ ({ pick_3(1) }).call(10) }) == 11) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"pick(1)(10) == 11"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ ({ pick_3(-1_i32) }).call(10) }) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"pick(-1)(10) == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ ({ pick_3(0) }).call(10) }) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"pick(0)(10) == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ apply_4(FnPtr::<fn(i32) -> i32>::new(inc_0), 5) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"apply(inc, 5) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ apply_4(FnPtr::<fn(i32) -> i32>::null(), 5) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"apply(nullptr, 5) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_5(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
