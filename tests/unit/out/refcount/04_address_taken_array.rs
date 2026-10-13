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
    let arr2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 2])));
    (*arr2.borrow_mut())[(0) as usize] = 3;
    (*arr2.borrow_mut())[(1) as usize] = 4;
    let mut arr2_ptr: Ptr<i32> = (arr2.as_pointer() as Ptr<i32>);
    elem!(arr2_ptr, 0).write(5);
    elem!(arr2_ptr, 1).write(6);
    let arr2_ref1: Ptr<i32> = (arr2.as_pointer() as Ptr<i32>).offset(1);
    arr2_ref1.write(7);
    (if (((*arr2.borrow())[(0) as usize] + (*arr2.borrow())[(1) as usize]) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"arr2[0] + arr2[1] == 12"),
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
