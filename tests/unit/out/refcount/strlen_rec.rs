extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(mut s: Ptr<i8>, mut n: i32) -> i32 {
    return if ((s.read()) != 0) {
        ({ strlen_0(s.offset((1) as isize), (n + 1)) })
    } else {
        n
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('s' as i8),
        ('t' as i8),
        ('r' as i8),
        ('\0' as i8),
    ])));
    (if (({ strlen_0(((s.as_pointer() as Ptr<i8>).offset(0)), 0) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"strlen(&s[0], 0) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
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
