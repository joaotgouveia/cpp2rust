extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Pod {
    #[offset(0)]
    pub v: i32,
}
pub fn zero_0() -> Ptr<i32> {
    return Ptr::<i32>::null();
}
pub fn zero_1() -> i64 {
    return 0_i64;
}
pub fn destroy_2(mut p: Ptr<i32>) {}
pub fn destroy_3(mut p: Ptr<Pod>) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut i: i32 = 0_i32;
    let mut d: f64 = 0_f64;
    let mut p: Ptr<i32> = ({ zero_0() });
    (if (i == 0) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"i == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (d == 0.0E+0) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"d == 0.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (p).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"p == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ zero_1() }) == 0_i64) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"zero<long>() == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let x: Value<i32> = Rc::new(RefCell::new(5));
    ({ destroy_2((x.as_pointer())) });
    (if ((*x.borrow()) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"x == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let pod: Value<Pod> = Rc::new(RefCell::new(Pod { v: 7 }));
    ({ destroy_3((pod.as_pointer())) });
    (if ({ (*pod.borrow()).v } == 7) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"pod.v == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_4(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
