extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Elem {
    #[offset(0)]
    pub id: (),
    #[offset(4)]
    #[byte_size(16)]
    pub hist: Box<[()]>,
    #[offset(24)]
    #[byte_size(24)]
    pub coeffs: (),
}
impl Default for Elem {
    fn default() -> Self {
        {
            Elem {
                id: unimplemented!(),
                hist: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                coeffs: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(88)]
pub struct State {
    #[offset(0)]
    pub count: (),
    #[offset(4)]
    #[byte_size(24)]
    pub grid: Box<[Box<[()]>]>,
    #[offset(28)]
    #[byte_size(12)]
    pub arr: (),
    #[offset(40)]
    #[byte_size(24)]
    pub elems: (),
    #[offset(64)]
    #[byte_size(24)]
    pub rows: (),
}
impl Default for State {
    fn default() -> Self {
        {
            State {
                count: unimplemented!(),
                grid: (0..2)
                    .map(|_| (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>())
                    .collect::<Box<[Box<[()]>]>>(),
                arr: unimplemented!(),
                elems: unimplemented!(),
                rows: unimplemented!(),
            }
        }
    }
}
pub fn sum_0(mut p: Ptr<()>, mut n: ()) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while (i < n) {
        s += { (elem!(p, i).read()) };
        i.prefix_inc();
    }
    return s;
}
pub fn fill_1(mut s: Ptr<State>) {
    let mut i: () = 0_();
    'loop_: while (i < 2_()) {
        let mut j: () = 0_();
        'loop_: while (j < 3_()) {
            index!(index!(field!(s, grid), i), j).write({ ((i * 3_()) + j) });
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    unimplemented!().write(7_());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 3_()) {
        let __rhs = i;
        field!(unimplemented!(), id).write(__rhs);
        let __rhs = (10_() * i);
        array_elem!(unimplemented!(), hist, i).write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<State> = Rc::new(RefCell::new(<State>::default()));
    ({ fill_1((s.as_pointer())) });
    unimplemented!();
    unimplemented!();
    let mut e: Ptr<Elem> = (unimplemented!());
    let mut h: Ptr<()> = (array_field_ptr!(e, hist) as Ptr<()>);
    let mut c: Ptr<()> = (unimplemented!());
    let mut id: Ptr<()> = (field_ptr!(unimplemented!(), id));
    let mut k: () = 0_();
    'loop_: while (k < 4_()) {
        {
            let __rhs = { k };
            elem!(h, k).with_mut(|__v| *__v = *__v + __rhs)
        };
        k.prefix_inc();
    }
    {
        c.with_mut(|__v| *__v = *__v + 5_())
    };
    id.write(11_());
    (*s.borrow_mut()).count = 3_();
    unimplemented!();
    unimplemented!();
    let mut row: Ptr<()> = (unimplemented!());
    elem!(row, 2_()).write(9_());
    unimplemented!();
    let t: Value<State> = Rc::new(RefCell::new((*s.borrow()).clone()));
    array_elem!(unimplemented!(), hist, 0_()).write(100_());
    (*t.borrow_mut()).grid[(0_()) as usize][(0_()) as usize] = 100_();
    unimplemented!().write(100_());
    unimplemented!();
    unimplemented!();
    let mut p: Ptr<State> = (t.as_pointer());
    let __rhs = ({ (*p.upgrade().deref()).grid[(1_()) as usize][(1_()) as usize] } + {
        (unimplemented!().read())
    });
    array_elem!(unimplemented!(), hist, 3_()).write(__rhs);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
