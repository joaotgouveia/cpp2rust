extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static a_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let next: Value<lambda_1> = Rc::new(RefCell::new((lambda_1 {})));
    (if (({ lambda_1::operator_call() }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"next() == 1"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                10_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ lambda_1::operator_call() }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"next() == 2"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                11_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let copy: Value<lambda_1> = Rc::new(RefCell::new((*next.borrow()).clone()));
    (if (({ lambda_1::operator_call() }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"copy() == 3"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                14_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ lambda_1::operator_call() }) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"next() == 4"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                15_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let per_type: Value<lambda_4> = Rc::new(RefCell::new((lambda_4 {})));
    (if (({ lambda_4::operator_call_5(1) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"per_type(1) == 1"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                22_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ lambda_4::operator_call_5(2) }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"per_type(2) == 2"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                23_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ lambda_4::operator_call_7(1.0E+0) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"per_type(1.0) == 1"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                24_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ lambda_4::operator_call_5(3) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"per_type(3) == 3"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                25_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let read: Value<lambda_9> = Rc::new(RefCell::new((lambda_9 {})));
    (if (({ lambda_9::operator_call(0) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"read(0) == 0"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                28_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    a_0.with(|rc| *rc.borrow_mut() = 1);
    (if (({ lambda_9::operator_call(0) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"read(0) == 1"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                30_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let local: Value<lambda_10> = Rc::new(RefCell::new((lambda_10 {})));
    (if (({ lambda_10::operator_call() }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<u8>::from_string_literal(b"local() == 6"),
                Ptr::<u8>::from_string_literal(b"lambda_static_local.cpp"),
                40_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct lambda_1 {}
impl lambda_1 {
    pub fn operator_call() -> i32 {
        thread_local!(
            static n_2: Value<i32> = Rc::new(RefCell::new(0));
        );
        return (*n_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Callable0<i32> for lambda_1 {
    fn call(&self) -> i32 {
        { lambda_1::operator_call() }
    }
}
impl lambda_1 {
    pub fn to_free_function(&self) -> FnPtr<fn() -> i32> {
        FnPtr::new(lambda_1::operator_call)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct lambda_4 {}
impl lambda_4 {
    pub fn operator_call_5(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        thread_local!(
            static calls_6: Value<i32> = Rc::new(RefCell::new(0));
        );
        (*calls_6.with(Value::clone).borrow_mut()).postfix_inc();
        return calls_6.with(|rc| *rc.borrow());
    }
    pub fn operator_call_7(x: f64) -> i32 {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        thread_local!(
            static calls_8: Value<i32> = Rc::new(RefCell::new(0));
        );
        (*calls_8.with(Value::clone).borrow_mut()).postfix_inc();
        return calls_8.with(|rc| *rc.borrow());
    }
}
impl Callable1<i32, i32> for lambda_4 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_4::operator_call_5(a1) }
    }
}
impl Callable1<f64, i32> for lambda_4 {
    fn call(&self, a1: f64) -> i32 {
        { lambda_4::operator_call_7(a1) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct lambda_9 {}
impl lambda_9 {
    pub fn operator_call(_a0: i32) -> i32 {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        return a_0.with(|rc| *rc.borrow());
    }
}
impl Callable1<i32, i32> for lambda_9 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_9::operator_call(a1) }
    }
}
impl lambda_9 {
    pub fn to_free_function(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_9::operator_call)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct P_11 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct lambda_10 {}
impl lambda_10 {
    pub fn operator_call() -> i32 {
        let p: Value<P_11> = Rc::new(RefCell::new(P_11 { x: 2, y: 3 }));
        return ({ (*p.borrow()).x } * { (*p.borrow()).y });
    }
}
impl Callable0<i32> for lambda_10 {
    fn call(&self) -> i32 {
        { lambda_10::operator_call() }
    }
}
impl lambda_10 {
    pub fn to_free_function(&self) -> FnPtr<fn() -> i32> {
        FnPtr::new(lambda_10::operator_call)
    }
}
pub fn __assert_fail_3(__assertion: Ptr<u8>, __file: Ptr<u8>, __line: u32, __function: Ptr<u8>) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = a_0.with(|_| ());
}
