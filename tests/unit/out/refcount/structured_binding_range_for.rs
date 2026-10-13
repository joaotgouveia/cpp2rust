extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Pair]>> = Rc::new(RefCell::new(Box::new([
        Pair {
            first: 1,
            second: 2,
        },
        Pair {
            first: 3,
            second: 4,
        },
        Pair {
            first: 5,
            second: 6,
        },
    ])));
    {
        let __range1: Ptr<Pair> = arr.as_pointer();
        let mut __begin1: Ptr<Pair> = (__range1).clone();
        let mut __end1: Ptr<Pair> = (__range1).offset((3_i64) as isize);
        'loop_: while ({ (__begin1).clone() } != { (__end1).clone() }) {
            let __decomp_0: Ptr<Pair> = (__begin1).clone();
            {
                let __rhs = __decomp_0.with(|__s| __s.second);
                field!(__decomp_0, first).with_mut(|__v| *__v = *__v + __rhs)
            };
            __begin1.prefix_inc();
        }
    }
    (if ({ (*arr.borrow())[(0) as usize].first } == 3) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"arr[0].first == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*arr.borrow())[(2) as usize].first } == 11) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"arr[2].first == 11"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    {
        let __range1: Ptr<Pair> = arr.as_pointer();
        let mut __begin1: Ptr<Pair> = (__range1).clone();
        let mut __end1: Ptr<Pair> = (__range1).offset((3_i64) as isize);
        'loop_: while ({ (__begin1).clone() } != { (__end1).clone() }) {
            let __decomp_2: Value<Pair> =
                Rc::new(RefCell::new((*__begin1.upgrade().deref()).clone()));
            (*__decomp_2.borrow_mut()).first = 0;
            (*__decomp_2.borrow_mut()).second = 0;
            __begin1.prefix_inc();
        }
    }
    (if ({ (*arr.borrow())[(1) as usize].first } == 7) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"arr[1].first == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*arr.borrow())[(1) as usize].second } == 4) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"arr[1].second == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut sum: i32 = 0;
    {
        let __range1: Ptr<Pair> = arr.as_pointer();
        let mut __begin1: Ptr<Pair> = (__range1).clone();
        let mut __end1: Ptr<Pair> = (__range1).offset((3_i64) as isize);
        'loop_: while ({ (__begin1).clone() } != { (__end1).clone() }) {
            let __decomp_3: Ptr<Pair> = (__begin1).clone();
            sum += (__decomp_3.with(|__s| __s.first) * __decomp_3.with(|__s| __s.second));
            __begin1.prefix_inc();
        }
    }
    (if (sum == (((3 * 2) + (7 * 4)) + (11 * 6))) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"sum == 3 * 2 + 7 * 4 + 11 * 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
