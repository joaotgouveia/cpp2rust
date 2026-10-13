extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn negate_0(mut x: Ptr<i32>) {
    x.write({ -(x.read()) });
}
pub fn zero_out_1(mut x: Ptr<i32>) {
    x.write(0);
}
pub fn run_2(mut fn_: FnPtr<fn(Ptr<i32>)>, mut x: Ptr<i32>) {
    ({ fn_.call((x).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(42));
    ({ run_2(FnPtr::<fn(Ptr<i32>)>::new(negate_0), (a.as_pointer())) });
    (if ((*a.borrow()) == -42_i32) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a == -42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ run_2(FnPtr::<fn(Ptr<i32>)>::new(zero_out_1), (a.as_pointer())) });
    (if ((*a.borrow()) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"a == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut fn_: FnPtr<fn(Ptr<i32>)> = FnPtr::<fn(Ptr<i32>)>::new(negate_0);
    (if !((fn_).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"fn != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let b: Value<i32> = Rc::new(RefCell::new(10));
    ({ fn_.call((b.as_pointer())) });
    (if ((*b.borrow()) == -10_i32) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"b == -10"),
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
