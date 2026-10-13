extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(mut ptr: Ptr<i8>) -> u32 {
    let mut count: u32 = 0_u32;
    'loop_: while (((ptr.postfix_inc().read()) as i32) != (('\0' as i8) as i32)) {
        count.prefix_inc();
    }
    return count;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let string: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('h' as i8),
        ('e' as i8),
        ('l' as i8),
        ('l' as i8),
        ('o' as i8),
        ('\0' as i8),
    ])));
    (if (({ strlen_0(((string.as_pointer() as Ptr<i8>).offset(0))) }) == 5_u32) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"strlen(&string[0]) == 5"),
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
