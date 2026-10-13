extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub v: i32,
}
impl A {
    pub fn new_1() -> Self {
        Self { v: 1 }
    }
    pub fn new_2(mut v: i32) -> Self {
        Self { v: v }
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    pub v: i32,
}
impl B {
    pub fn new() -> Self {
        Self { v: 2 }
    }
}
impl Default for B {
    fn default() -> Self {
        { B::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoDefault {
    #[offset(0)]
    pub v: i32,
}
impl NoDefault {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
}
pub fn used_0(x: Option<A>) -> i32 {
    let mut x: A = x.unwrap_or_else(|| A::new_1());
    return x.v;
}
pub fn used_1(x: Option<B>) -> i32 {
    let mut x: B = x.unwrap_or_else(|| B::new());
    return x.v;
}
pub fn scaled_2(mut x: A, n: Option<i32>) -> i32 {
    let mut n: i32 = n.unwrap_or_else(|| (4usize as i32));
    return (x.v * n);
}
pub fn always_given_3(mut x: NoDefault) -> i32 {
    return x.v;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_4 {
    #[offset(0)]
    pub v: i32,
}
impl S_4 {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ used_0(None) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"used<A>() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ used_0(Some(A::new_2({ 5 }))) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"used(A(5)) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ used_1(None) }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"used<B>() == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ scaled_2(A::new_2({ 3 }), None) }) == (3 * (4usize as i32))) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"scaled(A(3)) == 3 * (int)sizeof(A)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ scaled_2(A::new_2({ 3 }), Some(2)) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"scaled(A(3), 2) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ always_given_3(NoDefault::new({ 3 })) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"always_given(NoDefault(3)) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let s: Value<S_4> = Rc::new(RefCell::new(S_4::new({ 1 })));
    (if (({ S_4Impl::get(&s.as_pointer(), NoDefault::new({ 4 })) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"s.get(NoDefault(4)) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_5(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait S_4Impl {
    fn get(&self, t: NoDefault) -> i32;
}
impl S_4Impl for Ptr<S_4> {
    fn get(&self, mut t: NoDefault) -> i32 {
        return ((*self).with(|__s| __s.v) + t.v);
    }
}
pub fn __cpp2rust_init_globals() {}
