extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(72)]
pub struct Pair {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
    #[offset(8)]
    #[byte_size(20)]
    pub a: Box<[()]>,
    #[offset(32)]
    #[byte_size(8)]
    pub r: Ptr<()>,
    #[offset(40)]
    #[byte_size(8)]
    pub p: Ptr<()>,
    #[offset(48)]
    #[byte_size(8)]
    pub pair: Ptr<Pair>,
    #[offset(56)]
    #[byte_size(16)]
    pub ap: Box<[Ptr<()>]>,
}
impl Default for Pair {
    fn default() -> Self {
        {
            Pair {
                x: unimplemented!(),
                y: unimplemented!(),
                a: (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                r: <Ptr<()>>::default(),
                p: Ptr::<()>::null(),
                pair: Ptr::<Pair>::null(),
                ap: (0..2)
                    .map(|_| Ptr::<()>::null())
                    .collect::<Box<[Ptr<()>]>>(),
            }
        }
    }
}
pub fn zero_0() -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X1 {}
pub fn foo_1(mut x1: (), x2: Ptr<()>, mut x3: Ptr<()>, p2: Ptr<Pair>, mut p3: Ptr<Pair>) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x1: Value<()> = Rc::new(RefCell::new(1_()));
    let c1: Value<()> = Rc::new(RefCell::new((*x1.borrow())));
    let rx1: Ptr<()> = x1.as_pointer();
    let px1: Value<Ptr<()>> = Rc::new(RefCell::new((x1.as_pointer())));
    let x2: Value<()> = Rc::new(RefCell::new((rx1.read())));
    let rx2: Ptr<()> = (rx1).clone();
    let mut px2: Ptr<()> = (rx1).clone();
    let mut x3: () = ((*px1.borrow()).read());
    let rx3: Ptr<()> = (*px1.borrow()).clone();
    let mut px3: Ptr<()> = (*px1.borrow()).clone();
    let mut res: () = ((*x1.borrow()) + (*x2.borrow()));
    res = ((*x1.borrow()) + (*x2.borrow()));
    let y1: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: 1_(),
        y: 2_(),
        a: Box::new([1_(), 2_(), 3_(), 4_(), 5_()]),
        r: x1.as_pointer(),
        p: Ptr::<()>::null(),
        pair: Ptr::<Pair>::null(),
        ap: Box::new([Ptr::<()>::null(), Ptr::<()>::null()]),
    }));
    let mut y4: Pair = Pair {
        x: { (*y1.borrow()).x },
        y: { (*y1.borrow()).y },
        a: Box::new([
            (array_elem!(y1.as_pointer(), a, unimplemented!()).read()),
            (array_elem!(y1.as_pointer(), a, 1_()).read()),
            (array_elem!(y1.as_pointer(), a, 2_()).read()),
            (array_elem!(y1.as_pointer(), a, 3_()).read()),
            (array_elem!(y1.as_pointer(), a, 4_()).read()),
        ]),
        r: ({ (*y1.borrow()).r.clone() }).clone(),
        p: { (*y1.borrow()).p.clone() },
        pair: { (*y1.borrow()).pair.clone() },
        ap: Box::new([
            (array_elem!(y1.as_pointer(), ap, unimplemented!()).read()),
            (array_elem!(y1.as_pointer(), ap, 1_()).read()),
        ]),
    };
    let ry1: Ptr<Pair> = y1.as_pointer();
    let py1: Value<Ptr<Pair>> = Rc::new(RefCell::new((y1.as_pointer())));
    let y2: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: ry1.with(|__s| __s.x),
        y: ry1.with(|__s| __s.y),
        a: Box::new([
            (array_elem!(ry1, a, unimplemented!()).read()),
            (array_elem!(ry1, a, 1_()).read()),
            (array_elem!(ry1, a, 2_()).read()),
            (array_elem!(ry1, a, 3_()).read()),
            (array_elem!(ry1, a, 4_()).read()),
        ]),
        r: (ry1.with(|__s| __s.r.clone())).clone(),
        p: ry1.with(|__s| __s.p.clone()),
        pair: ry1.with(|__s| __s.pair.clone()),
        ap: Box::new([
            (array_elem!(ry1, ap, unimplemented!()).read()),
            (array_elem!(ry1, ap, 1_()).read()),
        ]),
    }));
    let ry2: Ptr<Pair> = (ry1).clone();
    let mut py2: Ptr<Pair> = (ry1).clone();
    let y3: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: (*py1.borrow()).with(|__s| __s.x),
        y: (*py1.borrow()).with(|__s| __s.y),
        a: Box::new([
            (array_elem!((*py1.borrow()), a, unimplemented!()).read()),
            (array_elem!((*py1.borrow()), a, 1_()).read()),
            (array_elem!((*py1.borrow()), a, 2_()).read()),
            (array_elem!((*py1.borrow()), a, 3_()).read()),
            (array_elem!((*py1.borrow()), a, 4_()).read()),
        ]),
        r: ((*py1.borrow()).with(|__s| __s.r.clone())).clone(),
        p: (*py1.borrow()).with(|__s| __s.p.clone()),
        pair: (*py1.borrow()).with(|__s| __s.pair.clone()),
        ap: Box::new([
            (array_elem!((*py1.borrow()), ap, unimplemented!()).read()),
            (array_elem!((*py1.borrow()), ap, 1_()).read()),
        ]),
    }));
    let ry3: Ptr<Pair> = (*py1.borrow()).clone();
    let mut py3: Ptr<Pair> = (*py1.borrow()).clone();
    py3 = Ptr::<Pair>::null();
    let mut ptr2pair: Ptr<Pair> = (py3).clone();
    ({
        let _x1: () = (*x1.borrow());
        let _x2: Ptr<()> = x1.as_pointer();
        let _x3: Ptr<()> = (x1.as_pointer());
        let _p2: Ptr<Pair> = y1.as_pointer();
        let _p3: Ptr<Pair> = (y1.as_pointer());
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    ({
        let _x1: () = (rx1.read());
        let _x2: Ptr<()> = (rx1).clone();
        let _x3: Ptr<()> = (rx1).clone();
        let _p2: Ptr<Pair> = (ry1).clone();
        let _p3: Ptr<Pair> = (ry1).clone();
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    ({
        let _x1: () = ((*px1.borrow()).read());
        let _x2: Ptr<()> = (*px1.borrow()).clone();
        let _x3: Ptr<()> = (*px1.borrow()).clone();
        let _p2: Ptr<Pair> = (*py1.borrow()).clone();
        let _p3: Ptr<Pair> = (*py1.borrow()).clone();
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    let cr1: Ptr<()> = c1.as_pointer();
    let mut cp1: Ptr<()> = (c1.as_pointer());
    (*x1.borrow_mut()) = (*c1.borrow());
    (*x1.borrow_mut()) = 1_();
    (*x1.borrow_mut()) = { (cr1.read()) };
    (*x1.borrow_mut()) = { (cp1.read()) };
    rx1.write({ (*c1.borrow()) });
    rx2.write({ (cr1.read()) });
    rx3.write({ (cp1.read()) });
    (*px1.borrow()).write({ (*c1.borrow()) });
    px2.write({ (cr1.read()) });
    px3.write({ (cp1.read()) });
    (*px1.borrow_mut()) = (c1.as_pointer());
    px2 = (cr1).clone();
    px3 = (cp1).clone();
    (*y1.borrow_mut()).x = 2_();
    (*y1.borrow_mut()).y = 3_();
    array_elem!(y1.as_pointer(), a, unimplemented!()).write(100_());
    { (*y1.borrow()).r.clone() }.write(10_());
    (*y1.borrow_mut()).p = (px3).clone();
    px3 = (px2).clone();
    (*y1.borrow_mut()).pair = (y3.as_pointer());
    field!({ (*y1.borrow()).pair.clone() }, x).write(100_());
    field!({ (*y1.borrow()).pair.clone() }, pair).write((y2.as_pointer()));
    field!(
        { (*y1.borrow()).pair.clone() }.with(|__s| (__s).pair.clone()),
        x
    )
    .write(100_());
    array_elem!(y1.as_pointer(), ap, unimplemented!()).write((x1.as_pointer()));
    array_elem!(y1.as_pointer(), ap, 1_()).write((x2.as_pointer()));
    (array_elem!(y1.as_pointer(), ap, unimplemented!()).read()).write(unimplemented!());
    (*c1.borrow_mut()) = ((*x1.borrow()) + 1_());
    let j: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut new_y: Pair = Pair {
        x: 1_(),
        y: 2_(),
        a: Box::new([1_(), 2_(), 3_(), 4_(), 5_()]),
        r: j.as_pointer(),
        p: Ptr::<()>::null(),
        pair: Ptr::<Pair>::null(),
        ap: Box::new([Ptr::<()>::null(), Ptr::<()>::null()]),
    };
    (*y1.borrow_mut()).x = { new_y.x };
    let mut i: () = 1_();
    array_elem!(y1.as_pointer(), a, i).write(-1_());
    (*x1.borrow_mut()).postfix_inc();
    (*x1.borrow_mut()).prefix_inc();
    (*y1.borrow_mut()).x.postfix_inc();
    field!({ (*y1.borrow()).pair.clone() }, pair).write((y2.as_pointer()));
    field!(
        { (*y1.borrow()).pair.clone() }.with(|__s| __s.pair.clone()),
        x
    )
    .write(10_());
    ({ PairImpl::method(&y1.as_pointer()) });
    (*y1.borrow_mut()).pair = (y2.as_pointer());
    (*y2.borrow_mut()).pair = (y3.as_pointer());
    ({ PairImpl::method(&{ (*y1.borrow()).pair.clone() }.with(|__s| __s.pair.clone())) });
    let mut x: X1 = <X1>::default();
    let mut y: X1 = <X1>::default();
    (*x1.borrow_mut()) = (({ zero_0() }) + { (*y1.borrow()).x });
    (*y1.borrow_mut()).x = (({ zero_0() }) + 5_());
    let mut ptr2ptr_1: Ptr<Ptr<()>> = (px1.as_pointer());
    let mut ptr2ptr_2: Ptr<Ptr<Pair>> = (py1.as_pointer());
    return unimplemented!();
}
pub trait PairImpl {
    fn method(&self);
    fn as_val(&self) -> ();
    fn as_ref(&self) -> Ptr<()>;
    fn as_ptr(&self) -> Ptr<()>;
}
impl PairImpl for Ptr<Pair> {
    fn method(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_inc());
        field!((*self), y).with_mut(|__v| __v.prefix_inc());
        array_elem!((*self), a, 4_()).write(1_());
        (*self).with(|__s| __s.r.clone()).write(1_());
        field!((*self), p).write(Ptr::<()>::null());
        field!((*self), pair).write(Ptr::<Pair>::null());
        array_elem!((*self), ap, unimplemented!()).write(Ptr::<()>::null());
    }
    fn as_val(&self) -> () {
        return (*self).with(|__s| __s.x);
    }
    fn as_ref(&self) -> Ptr<()> {
        return field_ptr!((*self), x);
    }
    fn as_ptr(&self) -> Ptr<()> {
        return (field_ptr!((*self), x));
    }
}
pub fn __cpp2rust_init_globals() {}
