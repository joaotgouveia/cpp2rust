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
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: Ptr<i32> = Ptr::alloc(5);
    let mut out: i32 = (x.read());
    x.delete();
    (if (out == 5) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"out == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut y: Ptr<i32> = Ptr::alloc(Default::default());
    y.write(9);
    (if ((y.read()) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"*y == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    y.delete();
    let mut p: Ptr<Pair> = Ptr::alloc(<Pair>::default());
    field!(p, x).write(1);
    field!(p, y).write(2);
    (if (({ p.with(|__s| __s.x) } + { p.with(|__s| __s.y) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"p->x + p->y == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    p.delete();
    let mut nullpointer: Ptr<i32> = Ptr::<i32>::null();
    nullpointer.delete();
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
