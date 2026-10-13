extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn triple_it_1(mut x: i32) -> i32 {
    return (x * 3);
}
thread_local!(
    pub static g_op_2: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::null()));
);
pub fn set_op_3(mut fn_: FnPtr<fn(i32) -> i32>) {
    g_op_2.with(|rc| *rc.borrow_mut() = (fn_).clone());
}
pub fn call_op_4(mut x: i32) -> i32 {
    if !(*g_op_2.with(Value::clone).borrow()).is_null() {
        return ({ (*g_op_2.with(Value::clone).borrow()).call(x) });
    }
    return x;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ call_op_4(5) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_op(5) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ set_op_3(FnPtr::<fn(i32) -> i32>::new(double_it_0)) });
    (if !((*g_op_2.with(Value::clone).borrow()).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"g_op != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*g_op_2.with(Value::clone).borrow()).clone() } == {
        FnPtr::<fn(i32) -> i32>::new(double_it_0)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"g_op == double_it"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ call_op_4(5) }) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_op(5) == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ set_op_3(FnPtr::<fn(i32) -> i32>::new(triple_it_1)) });
    (if ({ (*g_op_2.with(Value::clone).borrow()).clone() } == {
        FnPtr::<fn(i32) -> i32>::new(triple_it_1)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"g_op == triple_it"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ call_op_4(5) }) == 15) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_op(5) == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ set_op_3(FnPtr::<fn(i32) -> i32>::null()) });
    (if (*g_op_2.with(Value::clone).borrow()).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"g_op == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ call_op_4(5) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_op(5) == 5"),
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
pub fn __cpp2rust_init_globals() {
    let _ = g_op_2.with(|_| ());
}
