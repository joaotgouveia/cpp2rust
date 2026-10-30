extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static next_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn bump_1(mut v: Ptr<()>, mut x: ()) -> () {
    {
        let __rhs = x;
        elem!((v), 0_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (*next_0.with(Value::clone).borrow_mut()).postfix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut items: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs = (i).wrapping_mul(2_());
        items[i] = __rhs;
        i.prefix_inc();
    }
    {
        let rhs_0 = (items[1_()]).wrapping_add(5_());
        items[1_()] = rhs_0
    };
    items[2_()].postfix_inc();
    unimplemented!();
    elem!(v.as_pointer(), { (*v.borrow())[0_()] }).write(3_());
    let __rhs = ({ (*v.borrow())[1_()] } + 4_());
    elem!(v.as_pointer(), { (*v.borrow())[1_()] }).write(__rhs);
    unimplemented!();
    let mut p: Ptr<()> = (v.as_pointer().offset(0_()));
    elem!(v.as_pointer(), (p.read())).write(9_());
    unimplemented!();
    elem!(v.as_pointer(), ({ bump_1((v.as_pointer()), 10_(),) })).write(2_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = next_0.with(|_| ());
}
