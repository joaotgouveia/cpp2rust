extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
pub fn operator_not_0(a: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) == 0);
}
pub fn operator_and_1(a: Ptr<S>, b: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) != 0) && (b.with(|__s| __s.v) != 0);
}
pub fn operator_or_2(a: Ptr<S>, b: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) != 0) || (b.with(|__s| __s.v) != 0);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
    let f: Value<S> = Rc::new(RefCell::new(S { v: 0 }));
    (if ({
        let _a: Ptr<S> = f.as_pointer();
        operator_not_0(_a)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"!f"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _a: Ptr<S> = t.as_pointer();
        operator_not_0(_a)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"!!t"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _a: Ptr<S> = t.as_pointer();
        let _b: Ptr<S> = t.as_pointer();
        operator_and_1(_a, _b)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"t && t"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _a: Ptr<S> = t.as_pointer();
        operator_and_1(_a, f.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"!(t && f)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _a: Ptr<S> = t.as_pointer();
        operator_or_2(_a, f.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"t || f"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _a: Ptr<S> = f.as_pointer();
        let _b: Ptr<S> = f.as_pointer();
        operator_or_2(_a, _b)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"!(f || f)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_3(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
