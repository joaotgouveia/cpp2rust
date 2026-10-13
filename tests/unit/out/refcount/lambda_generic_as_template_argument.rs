extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_int_0(fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>(0).call(x) });
}
pub fn apply_int_2(fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>(0).call(x) });
}
pub fn apply_int_4(fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>(0).call(x) });
}
pub fn apply_double_6(fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(f64) -> f64>(1).call(x) });
}
pub fn apply_double_7(fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(f64) -> f64>(1).call(x) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let scale: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |mut x: i32| -> i32 {
            return (x * (*factor.borrow()));
        },
        |mut x: f64| -> f64 {
            return (x * ((*factor.borrow()) as f64));
        }
    )));
    (if (({ apply_int_0((*scale.borrow()).copy_from(), 4) }) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"apply_int(scale, 4) == 12"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ apply_double_6((*scale.borrow()).copy_from(), 1.5E+0) }) == 4.5E+0) {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"apply_double(scale, 1.5) == 4.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        apply_int_2(
            lambda!(Generic, {}, |mut x: i32| -> i32 {
                return -x;
            }),
            9,
        )
    }) == -9_i32)
    {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"apply_int([](auto x) { return -x; }, 9) == -9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let offset: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |mut x: i32| -> i32 {
            return (x + (*factor.borrow()));
        },
        |mut x: f64| -> f64 {
            return (x + ((*factor.borrow()) as f64));
        }
    )));
    (if (({ apply_int_4((*offset.borrow()).copy_from(), 4) }) == 7) {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"apply_int(offset, 4) == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ apply_double_7((*offset.borrow()).copy_from(), 1.5E+0) }) == 4.5E+0) {
        &(0);
    } else {
        ({
            __assert_fail_8(
                Ptr::<i8>::from_string_literal(b"apply_double(offset, 1.5) == 4.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_8(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
