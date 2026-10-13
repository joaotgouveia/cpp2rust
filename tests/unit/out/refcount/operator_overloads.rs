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
pub fn operator_div_0(a: Ptr<S>, b: Ptr<S>) -> i32 {
    return ({ a.with(|__s| __s.v) } / { b.with(|__s| __s.v) });
}
pub fn operator_div_1(mut a: S, mut b: i32) -> i32 {
    return ((a.v / b) + 1);
}
pub fn operator_rem_2(mut a: S, mut b: S) -> i32 {
    return (a.v % b.v);
}
pub fn operator_rem_3(a: Ptr<S>, mut b: i32) -> i32 {
    return (({ a.with(|__s| __s.v) } % { b }) + 1);
}
pub fn operator_eq_4(mut a: i32, mut b: S) -> i32 {
    return if (a == b.v) { 4 } else { 0 };
}
pub fn operator_eq_5(mut a: i64, b: Ptr<S>) -> i32 {
    return if ({ a } == { (b.with(|__s| __s.v) as i64) }) {
        5
    } else {
        0
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 6 }));
    let t: Value<S> = Rc::new(RefCell::new(S { v: 4 }));
    (if (({ SImpl::operator_eq_1(&s.as_pointer(), 6) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(s == 6) == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_eq_2(&s.as_pointer(), 6_i64) }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(s == 6L) == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_eq_3(&s.as_pointer(), 6.0E+0) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(s == 6.0) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_eq_1(&s.as_pointer(), 7) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(s == 7) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_add(&s.as_pointer(), t.as_pointer()) }) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"s + t == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_sub(&s.as_pointer(), (*t.borrow()).clone()) }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"s - t == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_mul_6(&s.as_pointer(), t.as_pointer()) }) == 24) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"s * t == 24"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_mul_7(&s.as_pointer(), 2) }) == 13) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"s * 2 == 13"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: Ptr<S> = s.as_pointer();
        operator_div_0(_a, t.as_pointer())
    }) == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"s / t == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: S = (*s.borrow()).clone();
        operator_div_1(_a, 4)
    }) == 2)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"s / 4 == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: S = (*s.borrow()).clone();
        operator_rem_2(_a, (*t.borrow()).clone())
    }) == 2)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"s % t == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: Ptr<S> = s.as_pointer();
        operator_rem_3(_a, 4)
    }) == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"s % 4 == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ operator_eq_4(6, (*s.borrow()).clone()) }) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(6 == s) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ operator_eq_5(6_i64, s.as_pointer()) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(6L == s) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_6(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait SImpl {
    fn operator_eq_1(&self, o: i32) -> i32;
    fn operator_eq_2(&self, o: i64) -> i32;
    fn operator_eq_3(&self, o: f64) -> i32;
    fn operator_add(&self, o: Ptr<S>) -> i32;
    fn operator_sub(&self, o: S) -> i32;
    fn operator_mul_6(&self, o: Ptr<S>) -> i32;
    fn operator_mul_7(&self, o: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn operator_eq_1(&self, mut o: i32) -> i32 {
        return if ((*self).with(|__s| __s.v) == o) {
            1
        } else {
            0
        };
    }
    fn operator_eq_2(&self, mut o: i64) -> i32 {
        return if (((*self).with(|__s| __s.v) as i64) == o) {
            2
        } else {
            0
        };
    }
    fn operator_eq_3(&self, mut o: f64) -> i32 {
        return if (((*self).with(|__s| __s.v) as f64) == o) {
            3
        } else {
            0
        };
    }
    fn operator_add(&self, o: Ptr<S>) -> i32 {
        return ({ (*self).with(|__s| __s.v) } + { o.with(|__s| __s.v) });
    }
    fn operator_sub(&self, mut o: S) -> i32 {
        return ((*self).with(|__s| __s.v) - o.v);
    }
    fn operator_mul_6(&self, o: Ptr<S>) -> i32 {
        return ({ (*self).with(|__s| __s.v) } * { o.with(|__s| __s.v) });
    }
    fn operator_mul_7(&self, mut o: i32) -> i32 {
        return (((*self).with(|__s| __s.v) * o) + 1);
    }
}
pub fn __cpp2rust_init_globals() {}
