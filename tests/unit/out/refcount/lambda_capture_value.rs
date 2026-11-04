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
pub struct S {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn read_0(v: Ptr<i32>) -> i32 {
    return (v.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let scale: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |mut x: i32| -> i32 {
            return (x * (*factor.borrow()));
        }
    )));
    ({ cpp2rust_assert_fail_1((({ (*scale.borrow()).call(4) }) == 12)) });
    (*factor.borrow_mut()) = 100;
    ({ cpp2rust_assert_fail_1((({ (*scale.borrow()).call(4) }) == 12)) });
    let slot: Value<i32> = Rc::new(RefCell::new(7));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((slot.as_pointer())));
    let read_ptr: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let p: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
        },
        || -> i32 {
            return ((*p.borrow()).read());
        }
    )));
    (*slot.borrow_mut()) = 8;
    ({ cpp2rust_assert_fail_1((({ (*read_ptr.borrow()).call() }) == 8)) });
    let s: Value<S> = Rc::new(RefCell::new(S { x: 1, y: 2 }));
    let sum: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let s: Value<S> = Rc::new(RefCell::new((*s.borrow()).clone()));
        },
        || -> i32 {
            return ({ (*s.borrow()).x } + { (*s.borrow()).y });
        }
    )));
    (*s.borrow_mut()).x = 50;
    ({ cpp2rust_assert_fail_1((({ (*sum.borrow()).call() }) == 3)) });
    let mut base: i32 = 10;
    let shifted: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let y: Value<i32> = Rc::new(RefCell::new((base + 1)));
        },
        |mut x: i32| -> i32 {
            return (x + (*y.borrow()));
        }
    )));
    ({ cpp2rust_assert_fail_1((({ (*shifted.borrow()).call(5) }) == 16)) });
    let k: Value<i32> = Rc::new(RefCell::new(3));
    let by_copy: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<i32> = Rc::new(RefCell::new((*k.borrow())));
        },
        |mut x: i32| -> i32 {
            return (x + 3);
        }
    )));
    ({ cpp2rust_assert_fail_1((({ (*by_copy.borrow()).call(1) }) == 4)) });
    let by_copy_used: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<i32> = Rc::new(RefCell::new((*k.borrow())));
        },
        |mut x: i32| -> i32 {
            return (x + ({ read_0(k.as_pointer()) }));
        }
    )));
    ({ cpp2rust_assert_fail_1((({ (*by_copy_used.borrow()).call(1) }) == 4)) });
    let implicit_used: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<i32> = Rc::new(RefCell::new((*k.borrow())));
        },
        |mut x: i32| -> i32 {
            return (x + ({ read_0(k.as_pointer()) }));
        }
    )));
    ({ cpp2rust_assert_fail_1((({ (*implicit_used.borrow()).call(1) }) == 4)) });
    let by_ref: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Ptr<i32> = k.as_pointer();
        },
        |mut x: i32| -> i32 {
            return (x + 3);
        }
    )));
    ({ cpp2rust_assert_fail_1((({ (*by_ref.borrow()).call(1) }) == 4)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
