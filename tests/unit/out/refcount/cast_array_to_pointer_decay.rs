extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn deref_0(mut p: Ptr<i32>) -> i32 {
    return (p.read());
}
pub fn strlen_1(mut s: Ptr<i8>) -> i32 {
    let mut c: i32 = 0;
    'loop_: while ((s.postfix_inc().read()) != 0) {
        c.prefix_inc();
    }
    return c;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    (if ((({ deref_0((a.as_pointer() as Ptr<i32>)) })
        + ({ strlen_1((s.as_pointer() as Ptr<i8>)) }))
        == 4)
    {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"deref(a) + strlen(s) == 4"),
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
