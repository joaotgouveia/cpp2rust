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
pub struct Foo {
    #[offset(0)]
    pub x1: i32,
    #[offset(4)]
    pub x2: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo { x1: 1, x2: 2 },
        Foo { x1: 3, x2: 4 },
    ])));
    let mut p1: Ptr<i32> = (field_ptr!((arr.as_pointer() as Ptr<Foo>).offset(1), x1));
    let mut a: i32 = (p1.read());
    let mut p2: Ptr<Foo> = ((arr.as_pointer() as Ptr<Foo>).offset(0));
    (if (({ a } + { p2.with(|__s| __s.x2) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"a + p2->x2 == 5"),
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
