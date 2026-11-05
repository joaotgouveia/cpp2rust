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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let mut b: i32 = 42;
    let r: Ptr<i32> = ({
        let __lhs = a.as_pointer();
        __lhs.write({ b });
        __lhs
    });
    ({ cpp2rust_assert_fail_0(((*a.borrow()) == 42)) });
    ({ cpp2rust_assert_fail_0(({ (r).clone() } == { (a.as_pointer()) })) });
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((arr.as_pointer() as Ptr<i32>)));
    let pr: Ptr<Ptr<i32>> = ({
        let __lhs = p.as_pointer();
        {
            let _ptr = __lhs.clone();
            _ptr.write(_ptr.read() + 3)
        };
        __lhs
    });
    ({ cpp2rust_assert_fail_0((((pr.read()).read()) == 4)) });
    ({ cpp2rust_assert_fail_0(({ (pr).clone() } == { (p.as_pointer()) })) });
    ({
        let __lhs = a.as_pointer();
        __lhs.write({ b });
        __lhs
    })
    .write(5);
    ({ cpp2rust_assert_fail_0(((*a.borrow()) == 5)) });
    ({ cpp2rust_assert_fail_0((b == 42)) });
    r.write(7);
    ({ cpp2rust_assert_fail_0(((*a.borrow()) == 7)) });
    let mut i: i32 = 0;
    let q: Ptr<i32> = ({
        let __lhs = (arr.as_pointer() as Ptr<i32>).offset(i.postfix_inc());
        __lhs.write(9);
        __lhs
    });
    ({ cpp2rust_assert_fail_0((i == 1)) });
    ({
        cpp2rust_assert_fail_0(({ (q).clone() } == { ((arr.as_pointer() as Ptr<i32>).offset(0)) }))
    });
    ({ cpp2rust_assert_fail_0(((*arr.borrow())[(0) as usize] == 9)) });
    let s: Ptr<i32> = ({
        let __lhs = (arr.as_pointer() as Ptr<i32>).offset(i.postfix_inc());
        {
            __lhs.with_mut(|__v| *__v = *__v + 10)
        };
        __lhs
    });
    ({ cpp2rust_assert_fail_0((i == 2)) });
    ({
        cpp2rust_assert_fail_0(({ (s).clone() } == { ((arr.as_pointer() as Ptr<i32>).offset(1)) }))
    });
    ({ cpp2rust_assert_fail_0(((*arr.borrow())[(1) as usize] == 12)) });
    let mut k: i32 = 3;
    let t: Ptr<i32> = ({
        let __rhs = k.postfix_inc();
        let __lhs = a.as_pointer();
        __lhs.write({ __rhs });
        __lhs
    });
    ({ cpp2rust_assert_fail_0((k == 4)) });
    ({ cpp2rust_assert_fail_0(((*a.borrow()) == 3)) });
    ({ cpp2rust_assert_fail_0(({ (t).clone() } == { (a.as_pointer()) })) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
