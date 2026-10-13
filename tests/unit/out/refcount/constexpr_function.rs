extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn runtime_only_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn first_1(mut p: Ptr<i32>) -> i32 {
    return (p.read());
}
pub fn scaled_2(mut x: i32) -> i32 {
    if (x < 0) {
        return ({ runtime_only_0(-x) });
    }
    return x;
}
pub fn half_3(mut x: f64) -> f64 {
    return (x / 2.0E+0);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Flag {
    #[offset(0)]
    pub v: i32,
}
pub fn use_4(f: Flag) -> i32 {
    let f: Value<Flag> = Rc::new(RefCell::new(f));
    (if ({ FlagImpl::to_bool(&f.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"f"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int use(Flag)"),
            )
        });
    });
    return { (*f.borrow()).v };
}
pub fn checked_6(mut x: i32) -> i32 {
    (if (x > 0) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"x > 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int checked(int)"),
            )
        });
    });
    return (x + 1);
}
pub fn in_constant_context_7() -> bool {
    {}
    return (({ runtime_only_0(1) }) == 0);
}
const _: () = assert!(true, "in_constant_context()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct P {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut runtime_context: bool = ({ in_constant_context_7() });
    (if !(runtime_context) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"!runtime_context"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([7, 8])));
    (if (({ first_1((arr.as_pointer() as Ptr<i32>)) }) == 7) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"first(arr) == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ first_1((arr.as_pointer() as Ptr<i32>).offset((1) as isize)) }) == 8) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"first(arr + 1) == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ scaled_2(3) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"scaled(3) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ scaled_2(-3_i32) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"scaled(-3) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ half_3(5.0E+0) }) == 2.5E+0) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"half(5.0) == 2.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ checked_6(1) }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"checked(1) == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut c: i32 = ({ checked_6(4) });
    (if (c == 5) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"c == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ use_4(Flag { v: 2 }) }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"use(Flag{2}) == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut u: i32 = ({ use_4(Flag { v: 3 }) });
    (if (u == 3) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"u == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ptr: Ptr<i32> = (arr.as_pointer() as Ptr<i32>);
    (if !(ptr).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"ptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let p: Value<P> = Rc::new(RefCell::new(P { v: 9 }));
    (if (({ PImpl::get(&p.as_pointer()) }) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"p.get() == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut k: i32 = ({ scaled_2(4) });
    (if (k == 4) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"k == 4"),
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
pub trait FlagImpl {
    fn to_bool(&self) -> bool;
}
impl FlagImpl for Ptr<Flag> {
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.v) != 0);
    }
}
pub trait PImpl {
    fn get(&self) -> i32;
}
impl PImpl for Ptr<P> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
