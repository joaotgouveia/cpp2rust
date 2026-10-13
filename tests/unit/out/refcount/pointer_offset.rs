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
    let mut out: i32 = 0;
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 0])));
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(0));
    'loop_: while ((ptr.read()) != 0) {
        out += { (ptr.read()) };
        ptr.prefix_inc();
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(1));
    'loop_: while ((ptr.read()) != 4) {
        out += { (ptr.read()) };
        ptr.postfix_inc();
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(4));
    'loop_: while ((ptr.read()) != 1) {
        out += { (ptr.read()) };
        ptr.postfix_dec();
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(3));
    'loop_: while ((ptr.read()) != 2) {
        out += { (ptr.read()) };
        ptr.prefix_dec();
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(0));
    'loop_: while ((ptr.read()) != 0) {
        out += { (ptr.read()) };
        ptr = { ptr.offset((1) as isize) };
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(0));
    let mut i: i32 = 0;
    'loop_: while (i < 5) {
        out += { (elem!(ptr, i).read()) };
        i.prefix_inc();
    }
    (if (out == 51) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"out == 51"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut step: i64 = 2_i64;
    let mut offset: Ptr<i32> = (arr.as_pointer() as Ptr<i32>).offset((step) as isize);
    (if ({ (offset.read()) } == { (*arr.borrow())[(2) as usize] }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"*offset == arr[2]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    offset = { offset.offset(-((step) as isize)) };
    (if ({ (offset).clone() } == { ((arr.as_pointer() as Ptr<i32>).offset(0)) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"offset == &arr[0]"),
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
