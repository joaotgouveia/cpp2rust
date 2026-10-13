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
    let s: Value<S> = Rc::new(RefCell::new(S { v: 10 }));
    let cs: Value<S> = Rc::new(RefCell::new(S { v: 10 }));
    let vs: Value<S> = Rc::new(RefCell::new(S { v: 10 }));
    (if (({ SImpl::operator_add_1(&s.as_pointer(), 1) }) == 11) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"s + 1 == 11"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_add_2(&cs.as_pointer(), 1) }) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"cs + 1 == 12"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_add_3(&vs.as_pointer(), 1) }) == 13) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"vs + 1 == 13"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_sub_4(&s.as_pointer(), 1) }) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"s - 1 == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_sub_5(&Rc::new(RefCell::new(S { v: 10 })).as_pointer(), 1) }) == 8) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"S{10} - 1 == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_mul_6(&s.as_pointer(), 3) }) == 30) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"s * 3 == 30"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_mul_6(&cs.as_pointer(), 3) }) == 30) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"cs * 3 == 30"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_mul_7(&Rc::new(RefCell::new(S { v: 10 })).as_pointer(), 3) }) == 60) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"S{10} * 3 == 60"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_index_8(&s.as_pointer(), 2) }) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"s[2] == 12"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_index_9(&cs.as_pointer(), 2) }) == 112) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"cs[2] == 112"),
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
    fn operator_add_1(&self, a: i32) -> i32;
    fn operator_add_2(&self, a: i32) -> i32;
    fn operator_add_3(&self, a: i32) -> i32;
    fn operator_sub_4(&self, a: i32) -> i32;
    fn operator_sub_5(&self, a: i32) -> i32;
    fn operator_mul_6(&self, a: i32) -> i32;
    fn operator_mul_7(&self, a: i32) -> i32;
    fn operator_index_8(&self, i: i32) -> i32;
    fn operator_index_9(&self, i: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn operator_add_1(&self, mut a: i32) -> i32 {
        return ((*self).with(|__s| __s.v) + a);
    }
    fn operator_add_2(&self, mut a: i32) -> i32 {
        return (((*self).with(|__s| __s.v) + a) + 1);
    }
    fn operator_add_3(&self, mut a: i32) -> i32 {
        return (((*self).with(|__s| __s.v) + a) + 2);
    }
    fn operator_sub_4(&self, mut a: i32) -> i32 {
        return ((*self).with(|__s| __s.v) - a);
    }
    fn operator_sub_5(&self, mut a: i32) -> i32 {
        return (((*self).with(|__s| __s.v) - a) - 1);
    }
    fn operator_mul_6(&self, mut a: i32) -> i32 {
        return ((*self).with(|__s| __s.v) * a);
    }
    fn operator_mul_7(&self, mut a: i32) -> i32 {
        return (((*self).with(|__s| __s.v) * a) * 2);
    }
    fn operator_index_8(&self, mut i: i32) -> i32 {
        return ((*self).with(|__s| __s.v) + i);
    }
    fn operator_index_9(&self, mut i: i32) -> i32 {
        return (((*self).with(|__s| __s.v) + i) + 100);
    }
}
pub fn __cpp2rust_init_globals() {}
