extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct POD {
    #[offset(0)]
    pub x1: i32,
    #[offset(4)]
    pub x2: i32,
    #[offset(8)]
    pub x3: i32,
}
pub fn PODIncrement_0(pod: Ptr<POD>) {
    {
        field!(pod, x1).with_mut(|__v| *__v = *__v + 1)
    };
    {
        field!(pod, x2).with_mut(|__v| *__v = *__v + 2)
    };
    {
        field!(pod, x3).with_mut(|__v| *__v = *__v + 3)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p1: POD = POD {
        x1: 10,
        x2: 11,
        x3: 12,
    };
    let p2: Value<POD> = Rc::new(RefCell::new(POD {
        x1: p1.x1,
        x2: p1.x2,
        x3: p1.x3,
    }));
    ({ PODIncrement_0(p2.as_pointer()) });
    (if ((({ (*p2.borrow()).x1 } + { (*p2.borrow()).x2 }) + { (*p2.borrow()).x3 }) == 39) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"p2.x1 + p2.x2 + p2.x3 == 39"),
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
