extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static calls_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static xs_1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
);
pub fn pick_2(mut i: ()) -> Ptr<()> {
    (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (xs_1.with(|v| v.as_pointer()) as Ptr<()>).offset(i);
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct Bag {
    #[offset(0)]
    pub n: (),
}
impl Bag {
    pub fn new(mut n: ()) -> Self {
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
        operator_lt_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Bag {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_3(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_3(__rhs.clone(), __lhs.clone()))
    }
}
impl Clone for Bag {
    fn clone(&self) -> Self {
        let __src: Value<Bag> = Rc::new(RefCell::new(Bag { n: self.n.clone() }));
        Bag::copy_from(__src.as_pointer())
    }
}
pub fn operator_lt_3(a: Ptr<Bag>, b: Ptr<Bag>) -> () {
    return ({ a.with(|__s| __s.n) } < { b.with(|__s| __s.n) });
}
pub fn make_4(mut n: ()) -> Bag {
    return Bag::new({ n });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x1: Value<()> = Rc::new(RefCell::new(1_()));
    let x2: Value<()> = Rc::new(RefCell::new(2_()));
    let x3: Value<()> = Rc::new(RefCell::new(10_()));
    let x4: Value<()> = Rc::new(RefCell::new(20_()));
    let mut p1: Ptr<()> = (x1.as_pointer());
    let mut p2: Ptr<()> = (x2.as_pointer());
    let mut r1: () = (({ max_5(x1.as_pointer(), x2.as_pointer()) }).read());
    let mut r2: () = (({ min_6(x3.as_pointer(), x4.as_pointer()) }).read());
    let mut r3: () = (({
        let ___a: Ptr<()> = (p1).clone();
        let ___b: Ptr<()> = x2.as_pointer();
        max_5(___a, ___b)
    })
    .read());
    let mut r4: () = (({
        let ___a: Ptr<()> = (p2).clone();
        let ___b: Ptr<()> = x3.as_pointer();
        min_6(___a, ___b)
    })
    .read());
    let mut r5: () = (({
        let ___a: Value<()> = Rc::new(RefCell::new(30_()));
        let ___b: Value<()> = Rc::new(RefCell::new(40_()));
        max_5(___a.as_pointer(), ___b.as_pointer())
    })
    .read());
    unimplemented!();
    let values: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 7_(), 7_(), 3_()])));
    let mut max: Ptr<()> = ({
        let ___first: Ptr<()> = (values.as_pointer() as Ptr<()>);
        let ___last: Ptr<()> = (values.as_pointer() as Ptr<()>).offset((4_()) as isize);
        max_element_7(___first, ___last)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    calls_0.with(|rc| *rc.borrow_mut() = 0_());
    unimplemented!();
    unimplemented!();
    return 0_();
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
