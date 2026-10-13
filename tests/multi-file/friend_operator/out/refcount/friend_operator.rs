extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<S> = Rc::new(RefCell::new(S { a: 1, b: 2 }));
    let s2: Value<S> = Rc::new(RefCell::new(S { a: 1, b: 2 }));
    let s3: Value<S> = Rc::new(RefCell::new(S { a: 1, b: 3 }));
    (if ({
        let _x: Ptr<S> = s1.as_pointer();
        operator_eq_0(_x, s2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s1 == s2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _x: Ptr<S> = s1.as_pointer();
        operator_ne_2(_x, s3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s1 != s3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _x: Ptr<S> = s1.as_pointer();
        operator_eq_0(_x, s3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"!(s1 == s3)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _x: Ptr<S> = s1.as_pointer();
        operator_lt_3(_x, s3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s1 < s3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _x: Ptr<S> = s3.as_pointer();
        operator_lt_3(_x, s1.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"!(s3 < s1)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ compare_4(s1.as_pointer(), s3.as_pointer()) }) == -1_i32) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"compare(s1, s3) == -1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ compare_4(s3.as_pointer(), s1.as_pointer()) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"compare(s3, s1) == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ compare_4(s1.as_pointer(), s2.as_pointer()) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"compare(s1, s2) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn operator_eq_0(x: Ptr<S>, y: Ptr<S>) -> bool {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) })
        && ({ x.with(|__s| __s.b) } == { y.with(|__s| __s.b) });
}
pub fn operator_ne_2(x: Ptr<S>, y: Ptr<S>) -> bool {
    return !({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_eq_0(_x, _y)
    });
}
pub fn operator_lt_3(x: Ptr<S>, y: Ptr<S>) -> bool {
    return ({ x.with(|__s| __s.a) } < { y.with(|__s| __s.a) })
        || (({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) })
            && ({ x.with(|__s| __s.b) } < { y.with(|__s| __s.b) }));
}
pub fn compare_4(x: Ptr<S>, y: Ptr<S>) -> i32 {
    if ({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_lt_3(_x, _y)
    }) {
        return -1_i32;
    }
    if ({
        let _x: Ptr<S> = (y).clone();
        let _y: Ptr<S> = (x).clone();
        operator_lt_3(_x, _y)
    }) {
        return 1;
    }
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
