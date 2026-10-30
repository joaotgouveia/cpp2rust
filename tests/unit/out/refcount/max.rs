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
    let mut x3: () = 10_();
    let mut x4: () = 20_();
    let mut p1: Ptr<()> = (x1.as_pointer());
    let mut p2: Ptr<()> = (x2.as_pointer());
    let mut r1: () = (unimplemented!().read());
    let mut r2: () = (unimplemented!().read());
    let mut r3: () = (unimplemented!().read());
    let mut r4: () = (unimplemented!().read());
    let mut r5: () = (unimplemented!().read());
    unimplemented!();
    let values: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 7_(), 7_(), 3_()])));
    let mut max: Ptr<()> = unimplemented!();
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
