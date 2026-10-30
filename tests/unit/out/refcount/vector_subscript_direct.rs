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
pub struct Point {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub values: (),
    #[offset(24)]
    #[byte_size(24)]
    pub points: (),
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            values: unimplemented!(),
            points: unimplemented!(),
        }
    }
}
pub fn push_and_index_0(v: Ptr<()>) -> () {
    unimplemented!();
    return (unimplemented!() - 1_());
}
pub fn sum_ref_1(v: Ptr<()>) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        s += (elem!(v, i).read());
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*v.borrow_mut())[0_()] = 10_();
    unimplemented!();
    (*v.borrow_mut())[1_()] += 5_();
    (*v.borrow_mut())[2_()].postfix_inc();
    unimplemented!();
    let __rhs = { (*v.borrow())[1_()] };
    (*v.borrow_mut())[0_()] = __rhs;
    unimplemented!();
    (*v.borrow_mut())[1_()] = 0_();
    unimplemented!();
    let mut i: () = 0_();
    elem!(v.as_pointer(), i.postfix_inc()).write(3_());
    unimplemented!();
    unimplemented!();
    elem!(v.as_pointer(), ({ push_and_index_0(v.as_pointer(),) })).write(5_());
    unimplemented!();
    let mut p: Ptr<()> = (v.as_pointer().offset(2_()));
    p.write(9_());
    unimplemented!();
    unimplemented!();
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    unimplemented!();
    elem!(field_ptr!(h.as_pointer(), values), 1_()).write(6_());
    unimplemented!();
    field!(elem!(field_ptr!(h.as_pointer(), points), 0_()), y).write(5_());
    unimplemented!();
    unimplemented!();
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let __rhs = ((elem!(field_ptr!(hp, values), 1_()).read()) + 1_());
    elem!(field_ptr!(hp, values), 0_()).write(__rhs);
    let __rhs = (elem!(field_ptr!(hp, values), 0_()).read());
    field!(elem!(field_ptr!(hp, points), 0_()), x).write(__rhs);
    unimplemented!();
    let mut q: Point = elem!(field_ptr!(h.as_pointer(), points), 0_()).read();
    q.x = 0_();
    unimplemented!();
    let grid: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    elem!(grid.as_pointer().offset(1_()), 2_()).write(8_());
    unimplemented!();
    let __rhs = (elem!(grid.as_pointer().offset(1_()), 2_()).read());
    elem!(grid.as_pointer().offset(0_()), 1_()).write(__rhs);
    let __rhs = (elem!(grid.as_pointer().offset(0_()), 1_()).read());
    {
        elem!(grid.as_pointer().offset(1_()), 0_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __rhs = ({ (*a.borrow())[0_()] } + { (*a.borrow())[2_()] });
    (*a.borrow_mut())[1_()] = __rhs;
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub trait PointImpl {
    fn sum(&self) -> ();
}
impl PointImpl for Ptr<Point> {
    fn sum(&self) -> () {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.y));
    }
}
pub fn __cpp2rust_init_globals() {}
