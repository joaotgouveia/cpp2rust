extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Foo {
    #[offset(0)]
    pub a_: i32,
    #[offset(4)]
    pub b_: i32,
}
impl Foo {
    pub fn new_1(mut a: i32, mut b: i32) -> Self {
        Self { a_: a, b_: b }
    }
    pub fn new_2(mut a: i32, mut count: Ptr<i32>) -> Self {
        let __this: Foo = Foo::new_1({ a }, { 0 });
        {
            count.with_mut(|__v| *__v = *__v + 1)
        };
        __this
    }
    pub fn new_3() -> Self {
        let __this: Value<Foo> = Rc::new(RefCell::new(Foo::new_1({ 7 }, { 8 })));
        let this: Ptr<Foo> = __this.as_pointer();
        {
            field!(this, a_).with_mut(|__v| *__v = *__v + 1)
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Foo {
    fn default() -> Self {
        { Foo::new_3() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let mut f: Foo = Foo::new_2({ 3 }, { (count.as_pointer()) });
    (if (f.a_ == 3) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"f.a_ == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (f.b_ == 0) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"f.b_ == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*count.borrow()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"count == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut g: Foo = Foo::new_3();
    (if (g.a_ == 8) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"g.a_ == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (g.b_ == 8) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"g.b_ == 8"),
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
