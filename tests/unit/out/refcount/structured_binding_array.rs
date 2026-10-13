extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let __decomp_0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 3, _>(
            |__i: usize| (*arr.borrow())[(__i) as usize],
        ))));
    (*__decomp_0.borrow_mut())[(0) as usize] = 10;
    (if ((*__decomp_0.borrow())[(0) as usize] == 10) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"a == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*__decomp_0.borrow())[(1) as usize] == 2) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"b == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*__decomp_0.borrow())[(2) as usize] == 3) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"c == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*arr.borrow())[(0) as usize] == 1) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"arr[0] == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let __decomp_2: Ptr<i32> = arr.as_pointer();
    elem!((__decomp_2), 0).write(7);
    {
        let __rhs = (elem!((__decomp_2), 1).read());
        elem!((__decomp_2), 2).with_mut(|__v| *__v = *__v + __rhs)
    };
    (if ((*arr.borrow())[(0) as usize] == 7) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"arr[0] == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*arr.borrow())[(2) as usize] == 5) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"arr[2] == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let darr: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([1.5E+0, 2.5E+0])));
    let __decomp_3: Value<Box<[f64]>> =
        Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
            |__i: usize| (*darr.borrow())[(__i) as usize],
        ))));
    (*__decomp_3.borrow_mut())[(0) as usize] = 4.0E+0;
    (if (((*__decomp_3.borrow())[(0) as usize] + (*__decomp_3.borrow())[(1) as usize]) == 6.5E+0) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"d0 + d1 == 6.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*darr.borrow())[(0) as usize] == 1.5E+0) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"darr[0] == 1.5"),
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
