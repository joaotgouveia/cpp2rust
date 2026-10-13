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
    pub v: u32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 12_u32 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 10_u32 }));
    (if ({ ({ SImpl::operator_bitnot(&a.as_pointer()) }).v } == !12_u32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(~a).v == ~0b1100u"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_bitand(&a.as_pointer(), b.as_pointer()) }).v } == 8_u32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a & b).v == 0b1000"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_bitor(&a.as_pointer(), b.as_pointer()) }).v } == 14_u32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a | b).v == 0b1110"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_bitxor(&a.as_pointer(), b.as_pointer()) }).v } == 6_u32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a ^ b).v == 0b0110"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_shl(&a.as_pointer(), 2) }).v } == 48_u32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a << 2).v == 0b110000"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_shr(&a.as_pointer(), 2) }).v } == 3_u32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a >> 2).v == 0b11"),
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
    fn operator_bitnot(&self) -> S;
    fn operator_bitand(&self, o: Ptr<S>) -> S;
    fn operator_bitor(&self, o: Ptr<S>) -> S;
    fn operator_bitxor(&self, o: Ptr<S>) -> S;
    fn operator_shl(&self, n: i32) -> S;
    fn operator_shr(&self, n: i32) -> S;
}
impl SImpl for Ptr<S> {
    fn operator_bitnot(&self) -> S {
        return S {
            v: !(*self).with(|__s| __s.v),
        };
    }
    fn operator_bitand(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } & { o.with(|__s| __s.v) }),
        };
    }
    fn operator_bitor(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } | { o.with(|__s| __s.v) }),
        };
    }
    fn operator_bitxor(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } ^ { o.with(|__s| __s.v) }),
        };
    }
    fn operator_shl(&self, mut n: i32) -> S {
        return S {
            v: ((*self).with(|__s| __s.v) << n),
        };
    }
    fn operator_shr(&self, mut n: i32) -> S {
        return S {
            v: ((*self).with(|__s| __s.v) >> n),
        };
    }
}
pub fn __cpp2rust_init_globals() {}
