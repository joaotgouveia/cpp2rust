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
    let N: Value<i32> = Rc::new(RefCell::new(5));
    let mut arr1: [i32; 5] = [0, 0, 0, 0, 0];
    let mut arr2: [i32; 5] = [1, 1, 1, 1, 1];
    let mut i: i32 = 0;
    'loop_: while (i < (*N.borrow())) {
        arr1[(i) as usize] = { (i + arr2[(i) as usize]) };
        i.prefix_inc();
    }
    let mut fatorial: i32 = 1;
    let mut i: i32 = 0;
    'loop_: while (i < (*N.borrow())) {
        fatorial *= arr1[(i) as usize];
        i.prefix_inc();
    }
    (if (fatorial == 120) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"fatorial == 120"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
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
