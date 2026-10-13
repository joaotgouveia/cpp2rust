extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut x: Ptr<i32>) {
    {
        x.with_mut(|__v| *__v = *__v * 2)
    };
}
pub fn maybe_call_1(mut cb: FnPtr<fn(Ptr<i32>)>, mut x: Ptr<i32>) {
    if !(cb).is_null() {
        ({ cb.call((x).clone()) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(5));
    ({ maybe_call_1(FnPtr::<fn(Ptr<i32>)>::new(double_it_0), (a.as_pointer())) });
    (if ((*a.borrow()) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"a == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let b: Value<i32> = Rc::new(RefCell::new(5));
    ({ maybe_call_1(FnPtr::<fn(Ptr<i32>)>::null(), (b.as_pointer())) });
    (if ((*b.borrow()) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"b == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut fn_: FnPtr<fn(Ptr<i32>)> = FnPtr::<fn(Ptr<i32>)>::null();
    if !(!(fn_).is_null()) {
        fn_ = FnPtr::<fn(Ptr<i32>)>::new(double_it_0);
    }
    let c: Value<i32> = Rc::new(RefCell::new(3));
    if !(fn_).is_null() {
        ({ fn_.call((c.as_pointer())) });
    }
    (if ((*c.borrow()) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"c == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
