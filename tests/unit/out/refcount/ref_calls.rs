extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn bar_0() -> i32 {
    return 1;
}
pub fn foo_1(x: Ptr<i32>) -> Ptr<i32> {
    return (x).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let y: Value<i32> = Rc::new(RefCell::new((({ foo_1(x.as_pointer()) }).read())));
    let z: Ptr<i32> = ({ foo_1(x.as_pointer()) });
    (if (({
        ({ ((({ foo_1(x.as_pointer()) }).read()) + (({ foo_1(y.as_pointer()) }).read())) } + {
            (({ foo_1((z).clone()) }).read())
        })
    } + { ({ bar_0() }) })
        == 16)
    {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"foo(x) + foo(y) + foo(z) + bar() == 16"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
