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
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let r: Ptr<i32> = {
        let __p = x.as_pointer();
        __p.with_mut(|__v| {
            __v.prefix_inc();
        });
        __p
    };
    r.write(5);
    ({ cpp2rust_assert_fail_0(((*x.borrow()) == 5)) });
    ({ cpp2rust_assert_fail_0(({ (r).clone() } == { (x.as_pointer()) })) });
    r.with_mut(|__v| {
        __v.prefix_dec();
    });
    ({ cpp2rust_assert_fail_0(((*x.borrow()) == 4)) });
    (*(*x.borrow_mut()).prefix_inc()) = 7;
    ({ cpp2rust_assert_fail_0(((*x.borrow()) == 7)) });
    let mut y: i32 = (*(*x.borrow_mut()).prefix_inc());
    ({ cpp2rust_assert_fail_0((y == 8) && ((*x.borrow()) == 8)) });
    let counter: Value<u32> = Rc::new(RefCell::new(0_u32));
    let pre: Ptr<u32> = {
        let __p = counter.as_pointer();
        __p.with_mut(|__v| {
            __v.prefix_dec();
        });
        __p
    };
    ({
        cpp2rust_assert_fail_0(
            ({ (pre).clone() } == { (counter.as_pointer()) }) && ((*counter.borrow()) == !0_u32),
        )
    });
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<i32>)));
    let q: Ptr<Ptr<i32>> = {
        let __p = p.as_pointer();
        __p.with_mut(|__v| {
            __v.prefix_inc();
        });
        __p
    };
    ({ cpp2rust_assert_fail_0(({ (q).clone() } == { (p.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0((((q.read()).read()) == 2)) });
    ({ cpp2rust_assert_fail_0((((*(*p.borrow_mut()).prefix_inc()).read()) == 3)) });
    let mut n: Ptr<i32> = (*(*p.borrow_mut()).prefix_dec()).clone();
    ({ cpp2rust_assert_fail_0(((n.read()) == 2)) });
    let z: Value<i32> = Rc::new(RefCell::new(7));
    let w: Ptr<i32> = {
        let __p = z.as_pointer();
        __p.with_mut(|__v| {
            __v.prefix_inc();
        });
        __p
    };
    w.write(1);
    ({ cpp2rust_assert_fail_0(((*z.borrow()) == 1)) });
    let mut sum: i32 = 0;
    let mut k: i32 = 0;
    'loop_: while (k < 3) {
        sum += (*(*z.borrow_mut()).prefix_inc());
        k.prefix_inc();
    }
    ({ cpp2rust_assert_fail_0((sum == 9) && ((*z.borrow()) == 4)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
