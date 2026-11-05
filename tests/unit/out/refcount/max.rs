extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static calls_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static xs_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
);
pub fn size_of_2(mut i: i32) -> u64 {
    (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return ((i * 3) as u64);
}
pub fn pick_3(mut i: i32) -> Ptr<i32> {
    (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (xs_1.with(|v| v.as_pointer()) as Ptr<i32>).offset(i);
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Bag {
    #[offset(0)]
    pub n: i32,
}
impl Bag {
    pub fn new(mut n: i32) -> Self {
        Self { n: n }
    }
    pub fn copy_from(o: Ptr<Bag>) -> Self {
        let __this: Bag = Self {
            n: o.with(|__s| __s.n),
        };
        (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for Bag {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Bag {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_4(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_4(__rhs.clone(), __lhs.clone()))
    }
}
impl Clone for Bag {
    fn clone(&self) -> Self {
        let __src: Value<Bag> = Rc::new(RefCell::new(Bag { n: self.n.clone() }));
        Bag::copy_from(__src.as_pointer())
    }
}
impl CppMove for Bag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_lt_4(a: Ptr<Bag>, b: Ptr<Bag>) -> bool {
    return ({ a.with(|__s| __s.n) } < { b.with(|__s| __s.n) });
}
pub fn make_5(mut n: i32) -> Bag {
    return Bag::new({ n });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new(2));
    let x3: Value<i32> = Rc::new(RefCell::new(10));
    let x4: Value<i32> = Rc::new(RefCell::new(20));
    let mut p1: Ptr<i32> = (x1.as_pointer());
    let mut p2: Ptr<i32> = (x2.as_pointer());
    let mut r1: i32 = (({ max_6(x1.as_pointer(), x2.as_pointer()) }).read());
    let mut r2: i32 = (({ min_7(x3.as_pointer(), x4.as_pointer()) }).read());
    let mut r3: i32 = (({
        let ___a: Ptr<i32> = (p1).clone();
        let ___b: Ptr<i32> = x2.as_pointer();
        max_6(___a, ___b)
    })
    .read());
    let mut r4: i32 = (({
        let ___a: Ptr<i32> = (p2).clone();
        let ___b: Ptr<i32> = x3.as_pointer();
        min_7(___a, ___b)
    })
    .read());
    let mut r5: i32 = (({
        let ___a: Value<i32> = Rc::new(RefCell::new(30));
        let ___b: Value<i32> = Rc::new(RefCell::new(40));
        max_6(___a.as_pointer(), ___b.as_pointer())
    })
    .read());
    ({ cpp2rust_assert_fail_8((((((r1 + r2) + r3) + r4) + r5) == 56)) });
    let values: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 7, 7, 3])));
    let mut max: Ptr<i32> = ({
        let ___first: Ptr<i32> = (values.as_pointer() as Ptr<i32>);
        let ___last: Ptr<i32> = (values.as_pointer() as Ptr<i32>).offset((4) as isize);
        max_element_9(___first, ___last)
    });
    ({
        cpp2rust_assert_fail_8(
            ({ (max).clone() } == { (values.as_pointer() as Ptr<i32>).offset((1) as isize) }),
        )
    });
    ({
        cpp2rust_assert_fail_8(
            ((({
                let ___a: Ptr<i32> = ({ pick_3(0) });
                let ___b: Ptr<i32> = ({ pick_3(1) });
                max_6(___a, ___b)
            })
            .read())
                == 2)
                && (calls_0.with(|rc| *rc.borrow()) == 2),
        )
    });
    ({
        cpp2rust_assert_fail_8(
            ((({
                let ___a: Ptr<i32> = ({ pick_3(1) });
                let ___b: Ptr<i32> = ({ pick_3(0) });
                min_7(___a, ___b)
            })
            .read())
                == 1)
                && (calls_0.with(|rc| *rc.borrow()) == 4),
        )
    });
    calls_0.with(|rc| *rc.borrow_mut() = 0);
    ({
        cpp2rust_assert_fail_8(
            (({
                let ___a: Value<Bag> = Rc::new(RefCell::new(({ make_5(1) })));
                let ___b: Value<Bag> = Rc::new(RefCell::new(({ make_5(2) })));
                max_10(___a.as_pointer(), ___b.as_pointer())
            })
            .with(|__s| __s.n)
                == 2)
                && (({
                    let ___a: Value<Bag> = Rc::new(RefCell::new(({ make_5(1) })));
                    let ___b: Value<Bag> = Rc::new(RefCell::new(({ make_5(2) })));
                    min_11(___a.as_pointer(), ___b.as_pointer())
                })
                .with(|__s| __s.n)
                    == 1),
        )
    });
    ({ cpp2rust_assert_fail_8((calls_0.with(|rc| *rc.borrow()) == 0)) });
    let largest: Value<u64> = Rc::new(RefCell::new(0_u64));
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        let __rhs = (({
            let ___b: Value<u64> = Rc::new(RefCell::new(({ size_of_2(i) })));
            max_12(largest.as_pointer(), ___b.as_pointer())
        })
        .read());
        (*largest.borrow_mut()) = __rhs;
        i.prefix_inc();
    }
    ({
        cpp2rust_assert_fail_8(
            ((*largest.borrow()) == 6_u64) && (calls_0.with(|rc| *rc.borrow()) == 3),
        )
    });
    return 0;
}
pub trait BagImpl {
    fn copy_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag>;
}
impl BagImpl for Ptr<Bag> {
    fn copy_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag> {
        field!((*self), n).write({ _a0.with(|__s| __s.n) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = calls_0.with(|_| ());
    let _ = xs_1.with(|_| ());
}
