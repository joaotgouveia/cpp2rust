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
impl S {}
pub trait Base {
    fn apply(&mut self, x: i32) -> i32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Derived {
    #[offset(8)]
    pub factor: i32,
}
impl Derived {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn pair_diff_0(mut p: Ptr<Pair>) -> i32 {
    return ({ p.with(|__s| __s.first) } - { p.with(|__s| __s.second) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
    (if (({ SImpl::get(&s.as_pointer()) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.get() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ SImpl::set(&s.as_pointer(), 4) });
    (if (({ SImpl::get(&s.as_pointer()) }) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.get() == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::add(&s.as_pointer(), 2) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.add(2) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let derived: Value<Derived> = Rc::new(RefCell::new(Derived::new({ 3 })));
    let mut base: PtrDyn<dyn Base> = (derived.as_pointer()).to_dyn::<dyn Base>(|w| w);
    (if (({ (*base.upgrade().deref_mut()).apply(5) }) == 15) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"base->apply(5) == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let pair: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: 7,
        second: 3,
    }));
    (if (({ pair_sum_2((pair.as_pointer())) }) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"pair_sum(&pair) == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ pair_diff_0((pair.as_pointer())) }) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"pair_diff(&pair) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
impl S {
    pub fn new(mut x: i32) -> Self {
        Self { v: x }
    }
}
impl Derived {
    pub fn new(mut factor: i32) -> Self {
        Self { factor: factor }
    }
}
impl S {}
impl Derived {}
pub fn pair_sum_2(mut p: Ptr<Pair>) -> i32 {
    return ({ p.with(|__s| __s.first) } + { p.with(|__s| __s.second) });
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
impl Base for Derived {
    fn apply(&mut self, mut x: i32) -> i32 {
        return ({ self.factor } * x);
    }
}
pub trait SImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn get(&self) -> i32;
    fn set(&self, mut x: i32) {
        unimplemented!()
    }
    fn add(&self, mut x: i32) -> i32 {
        unimplemented!()
    }
}
impl SImpl for Ptr<S> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
    fn destructor(&self) {}
    fn set(&self, mut x: i32) {
        field!((*self), v).write(x);
    }
    fn add(&self, mut x: i32) -> i32 {
        {
            let __rhs = x;
            field!((*self), v).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
