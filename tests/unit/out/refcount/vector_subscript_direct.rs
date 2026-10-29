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
    pub values: std_vector_int__std_allocator_int__,
    #[offset(24)]
    #[byte_size(24)]
    pub points: std_vector_Point__std_allocator_Point__,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            values: field!(_a0, values).with_mut(std::mem::take),
            points: field!(_a0, points).with_mut(std::mem::take),
        }
    }
}
pub fn push_and_index_0(v: Ptr<std_vector_int__std_allocator_int__>) -> () {
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(42_()));
        (*v.upgrade().deref()).push_back_58(___x.as_pointer())
    });
    return (({ (*v.upgrade().deref()).size() }) - 1_());
}
pub fn sum_ref_1(v: Ptr<std_vector_int__std_allocator_int__>) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ({ i } < { ({ (*v.upgrade().deref()).size() }) }) {
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
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
    ));
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
    ({ (*h.borrow()).values.resize_40(2_()) });
    elem!(field_ptr!(h.as_pointer(), values), 1_()).write(6_());
    ({
        let ___x: Value<Point> = Rc::new(RefCell::new(Point { x: 1_(), y: 2_() }));
        (*h.borrow()).points.push_back_58(___x.as_pointer())
    });
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
    let grid : Value<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____  >::default() )) ;
    ({
        let ___x: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(0_()));
            std_vector_int__std_allocator_int__::new_2({ 3_() }, { __tmp_0.as_pointer() }, None)
        }));
        (*grid.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(0_()));
            std_vector_int__std_allocator_int__::new_2({ 3_() }, { __tmp_1.as_pointer() }, None)
        }));
        (*grid.borrow()).push_back_58(___x.as_pointer())
    });
    elem!(grid.as_pointer().offset(1_()), 2_()).write(8_());
    unimplemented!();
    let __rhs = (elem!(grid.as_pointer().offset(1_()), 2_()).read());
    elem!(grid.as_pointer().offset(0_()), 1_()).write(__rhs);
    let __rhs = (elem!(grid.as_pointer().offset(0_()), 1_()).read());
    {
        elem!(grid.as_pointer().offset(1_()), 0_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    let a: Value<std_array_int__3_> = Rc::new(RefCell::new(unimplemented!()));
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
        field!((*self), values).write({ { (*_a0.upgrade().deref()).values.clone() } });
        field!((*self), points).write({ { (*_a0.upgrade().deref()).points.clone() } });
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
