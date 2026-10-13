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
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let mut p: Ptr<i32> = (x.as_pointer());
    {
        p.with_mut(|__v| *__v = *__v + 1)
    };
    (if ((*x.borrow()) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"x == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    p = ((a.as_pointer() as Ptr<i32>).offset(1));
    {
        p.with_mut(|__v| *__v = *__v + 1)
    };
    (if ((*a.borrow())[(0) as usize] == 1) && ((*a.borrow())[(1) as usize] == 3) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"a[0] == 1 && a[1] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    p.prefix_dec();
    {
        p.with_mut(|__v| *__v = *__v + 1)
    };
    (if ((*a.borrow())[(0) as usize] == 2) && ((*a.borrow())[(1) as usize] == 3) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"a[0] == 2 && a[1] == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    p = (x.as_pointer());
    {
        p.with_mut(|__v| *__v = *__v + 1)
    };
    (if ((*x.borrow()) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"x == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p2: Ptr<i32> = (p).clone();
    {
        p2.with_mut(|__v| *__v = *__v + 1)
    };
    (if ((*x.borrow()) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"x == 4"),
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
pub fn __cpp2rust_init_globals() {}
