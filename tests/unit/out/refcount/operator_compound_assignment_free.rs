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
pub fn operator_add_assign_0(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    field!(a, v).write({ (a.with(|__s| __s.v)).wrapping_add(b.with(|__s| __s.v)) });
    return (a).clone();
}
pub fn operator_sub_assign_1(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    field!(a, v).write({ (a.with(|__s| __s.v)).wrapping_sub(b.with(|__s| __s.v)) });
    return (a).clone();
}
pub fn operator_mul_assign_2(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    field!(a, v).write({ (a.with(|__s| __s.v)).wrapping_mul(b.with(|__s| __s.v)) });
    return (a).clone();
}
pub fn operator_div_assign_3(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    field!(a, v).write({ (a.with(|__s| __s.v)).wrapping_div(b.with(|__s| __s.v)) });
    return (a).clone();
}
pub fn operator_rem_assign_4(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    field!(a, v).write({ (a.with(|__s| __s.v)).wrapping_rem(b.with(|__s| __s.v)) });
    return (a).clone();
}
pub fn operator_bitand_assign_5(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let __rhs = { b.with(|__s| __s.v) };
        field!(a, v).with_mut(|__v| *__v = *__v & __rhs)
    };
    return (a).clone();
}
pub fn operator_bitor_assign_6(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let __rhs = { b.with(|__s| __s.v) };
        field!(a, v).with_mut(|__v| *__v = *__v | __rhs)
    };
    return (a).clone();
}
pub fn operator_bitxor_assign_7(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let __rhs = { b.with(|__s| __s.v) };
        field!(a, v).with_mut(|__v| *__v = *__v ^ __rhs)
    };
    return (a).clone();
}
pub fn operator_shl_assign_8(a: Ptr<S>, mut n: i32) -> Ptr<S> {
    {
        let __rhs = n;
        field!(a, v).with_mut(|__v| *__v = *__v << __rhs)
    };
    return (a).clone();
}
pub fn operator_shr_assign_9(a: Ptr<S>, mut n: i32) -> Ptr<S> {
    {
        let __rhs = n;
        field!(a, v).with_mut(|__v| *__v = *__v >> __rhs)
    };
    return (a).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 6_u32 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 4_u32 }));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_add_assign_0(_a, b.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 10_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_sub_assign_1(_a, b.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 6_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_mul_assign_2(_a, b.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 24_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 24"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_div_assign_3(_a, b.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 6_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_rem_assign_4(_a, b.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 2_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitor_assign_6(_a, b.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 6_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitand_assign_5(_a, b.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 4_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitxor_assign_7(_a, b.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 0_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (*a.borrow_mut()).v = 3_u32;
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_shl_assign_8(_a, 2)
    });
    (if ({ (*a.borrow()).v } == 12_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 12"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_shr_assign_9(_a, 1)
    });
    (if ({ (*a.borrow()).v } == 6_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _a: Ptr<S> = ({
            let _a: Ptr<S> = a.as_pointer();
            operator_add_assign_0(_a, b.as_pointer())
        });
        let _b: Ptr<S> = b.as_pointer();
        operator_add_assign_0(_a, _b)
    });
    (if ({ (*a.borrow()).v } == 14_u32) {
        &(0);
    } else {
        ({
            __assert_fail_10(
                Ptr::<i8>::from_string_literal(b"a.v == 14"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_10(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
