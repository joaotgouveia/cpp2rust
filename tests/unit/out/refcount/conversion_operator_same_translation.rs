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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct T {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 65 }));
    let mut a: i8 = ({ SImpl::to_i8(&s.as_pointer()) });
    let mut b: u8 = ({ SImpl::to_u8(&s.as_pointer()) });
    (if ((a as i32) == (('A' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"a == \'A\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((b as i32) == ((66 as u8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"b == u8\'B\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (((({ SImpl::to_i8(&s.as_pointer()) }) as i32)
        + (({ SImpl::to_u8(&s.as_pointer()) }) as i32))
        == 131)
    {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(
                    b"static_cast<char>(s) + static_cast<char8_t>(s) == 131",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let t: Value<T> = Rc::new(RefCell::new(T { v: 3 }));
    let mut c: i64 = ({ TImpl::to_i64_1(&t.as_pointer()) });
    let mut d: i64 = ({ TImpl::to_i64_2(&t.as_pointer()) });
    (if (c == 3_i64) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"c == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (d == 4_i64) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"d == 4"),
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
    fn to_i8(&self) -> i8;
    fn to_u8(&self) -> u8;
}
impl SImpl for Ptr<S> {
    fn to_i8(&self) -> i8 {
        return ((*self).with(|__s| __s.v) as i8);
    }
    fn to_u8(&self) -> u8 {
        return (((*self).with(|__s| __s.v) + 1) as u8);
    }
}
pub trait TImpl {
    fn to_i64_1(&self) -> i64;
    fn to_i64_2(&self) -> i64;
}
impl TImpl for Ptr<T> {
    fn to_i64_1(&self) -> i64 {
        return ((*self).with(|__s| __s.v) as i64);
    }
    fn to_i64_2(&self) -> i64 {
        return (((*self).with(|__s| __s.v) + 1) as i64);
    }
}
pub fn __cpp2rust_init_globals() {}
