extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn more_refs_0(x1: i32, x2: i32, r1: Ptr<i32>, r2: Ptr<i32>) {
    let x1: Value<i32> = Rc::new(RefCell::new(x1));
    let x2: Value<i32> = Rc::new(RefCell::new(x2));
    let rx1: Ptr<i32> = x1.as_pointer();
    let rx2: Ptr<i32> = x2.as_pointer();
    let mut pr1: Ptr<i32> = (r1).clone();
    let mut pr2: Ptr<i32> = (r2).clone();
    let rpr1: Ptr<i32> = (pr1).clone();
    let rpr2: Ptr<i32> = (pr2).clone();
    let r: Ptr<i32> = (r1).clone();
    {
        let __rhs = {
            ({
                ({
                    ({
                        ({ ({ ({ (1 + (rx1.read())) } + { (rx2.read()) }) } + { (pr1.read()) }) }
                            + { (pr2.read()) })
                    } + { (rpr1.read()) })
                } + { (rpr2.read()) })
            } + { (r.read()) })
        };
        rx2.with_mut(|__v| *__v = *__v + __rhs)
    };
    r1.write({ (rx2.read()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Val {
    #[offset(0)]
    pub x: i32,
}
pub fn sum_1(mut a: Val, mut b: Val) -> i32 {
    return (a.x + b.x);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new(2));
    ({ more_refs_0(3, 4, x1.as_pointer(), x2.as_pointer()) });
    (if (((*x1.borrow()) + (*x2.borrow())) == 21) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"x1 + x2 == 21"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let v: Value<Val> = Rc::new(RefCell::new(Val { x: 5 }));
    let mut acc: i32 = ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    acc += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    acc += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    acc += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    (if (acc == 40) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"acc == 40"),
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
