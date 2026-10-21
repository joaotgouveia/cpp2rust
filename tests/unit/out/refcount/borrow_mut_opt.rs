extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn convert_without_rhs_0() {
    let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut y: () = 1_();
    (*x.borrow_mut()) = unimplemented!();
    (*x.borrow_mut()) = (y + 1_());
    y = unimplemented!();
    y = ((*x.borrow()) + 1_());
    (*x.borrow_mut()) += 1_();
    y += 1_();
    y = unimplemented!();
    (*x.borrow_mut()) = unimplemented!();
    let mut z: () = ((*x.borrow()) + y);
    z = (((*x.borrow()) + y) + 1_());
    let mut arr: [(); 2] = [1_(), 2_()];
    let mut w: () = (arr[(y) as usize] + arr[(*x.borrow()) as usize]);
    w += ((z + y) + (*x.borrow()));
    let mut arr2: [(); 3] = [('a' as ()), ('b' as ()), ('c' as ())];
    let mut p1: Ptr<()> = (x.as_pointer());
    let mut c: () = arr2[(p1.read()) as usize];
    c = arr2[(p1.read()) as usize];
    let mut p2: Ptr<()> = (x.as_pointer());
    p2.write(1_());
    let r: Ptr<()> = x.as_pointer();
    r.write(1_());
}
pub fn convert_with_rhs_1() {
    let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*x.borrow_mut()) = { ((*x.borrow()) + 1_()) };
    let mut y: () = unimplemented!();
    y = { (y + 1_()) };
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
    (*arr.borrow_mut())[(y) as usize] = { (y + 1_()) };
    (*arr.borrow_mut())[(*x.borrow()) as usize] = { ((*x.borrow()) + 1_()) };
    (*arr.borrow_mut())[(*x.borrow()) as usize] = { ((*arr.borrow())[(y) as usize] + 1_()) };
    let z: Ptr<()> = x.as_pointer();
    (*x.borrow_mut()) += { (z.read()) };
    y += { (z.read()) };
    let mut p: Ptr<()> = (x.as_pointer());
    (*x.borrow_mut()) += { (p.read()) };
    y += { (p.read()) };
    p = ((arr.as_pointer() as Ptr<()>).offset(unimplemented!()));
    (*arr.borrow_mut())[(unimplemented!()) as usize] = { (p.read()) };
    {
        let __rhs = { (*x.borrow()) };
        z.with_mut(|__v| *__v = *__v + __rhs)
    };
    {
        let __rhs = { y };
        z.with_mut(|__v| *__v = *__v + __rhs)
    };
    {
        let __rhs = { (p.read()) };
        z.with_mut(|__v| *__v = *__v + __rhs)
    };
    {
        let __rhs = { (y + (*x.borrow())) };
        p.with_mut(|__v| *__v = *__v + __rhs)
    };
    {
        let __rhs = { ({ (*x.borrow()) } + { (z.read()) }) };
        p.with_mut(|__v| *__v = *__v + __rhs)
    };
    {
        let __rhs = { ({ y } + { (z.read()) }) };
        p.with_mut(|__v| *__v = *__v + __rhs)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ convert_without_rhs_0() });
    ({ convert_with_rhs_1() });
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
