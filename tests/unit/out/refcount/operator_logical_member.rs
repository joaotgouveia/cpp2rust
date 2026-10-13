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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
    let f: Value<S> = Rc::new(RefCell::new(S { v: 0 }));
    (if ({ SImpl::operator_not(&f.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"!f"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({ SImpl::operator_not(&t.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"!!t"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _o: Ptr<S> = t.as_pointer();
        SImpl::operator_and(&t.as_pointer(), _o)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"t && t"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({ SImpl::operator_and(&t.as_pointer(), f.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"!(t && f)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ SImpl::operator_or(&t.as_pointer(), f.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"t || f"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _o: Ptr<S> = f.as_pointer();
        SImpl::operator_or(&f.as_pointer(), _o)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"!(f || f)"),
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
pub trait SImpl {
    fn operator_not(&self) -> bool;
    fn operator_and(&self, o: Ptr<S>) -> bool;
    fn operator_or(&self, o: Ptr<S>) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_not(&self) -> bool {
        return ((*self).with(|__s| __s.v) == 0);
    }
    fn operator_and(&self, o: Ptr<S>) -> bool {
        return ((*self).with(|__s| __s.v) != 0) && (o.with(|__s| __s.v) != 0);
    }
    fn operator_or(&self, o: Ptr<S>) -> bool {
        return ((*self).with(|__s| __s.v) != 0) || (o.with(|__s| __s.v) != 0);
    }
}
pub fn __cpp2rust_init_globals() {}
