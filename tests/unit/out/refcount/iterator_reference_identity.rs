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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([10, 20, 30, 40])));
    let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().end()));
    let r: Ptr<RandomAccessIterator<i32>> = ({
        i.as_pointer().with_mut(|__it| {
            __it.iter_add_assign(-3_isize);
        });
        i.as_pointer()
    });
    assert!(({ (r).clone() } == { (i.as_pointer()) }));
    assert!((((*i.borrow()).iter_deref().read()) == 20));
    let s: Ptr<RandomAccessIterator<i32>> = {
        i.as_pointer().with_mut(|__it| {
            __it.iter_inc();
        });
        i.as_pointer()
    };
    assert!(({ (s).clone() } == { (i.as_pointer()) }));
    assert!((((*i.borrow()).iter_deref().read()) == 30));
    let t: Ptr<RandomAccessIterator<i32>> = {
        i.as_pointer().with_mut(|__it| {
            __it.iter_dec();
        });
        i.as_pointer()
    };
    assert!(({ (t).clone() } == { (i.as_pointer()) }));
    assert!((((*i.borrow()).iter_deref().read()) == 20));
    let u: Ptr<RandomAccessIterator<i32>> = ({
        i.as_pointer().with_mut(|__it| {
            __it.iter_add_assign(2_isize);
        });
        i.as_pointer()
    });
    assert!(({ (u).clone() } == { (i.as_pointer()) }));
    assert!((((*i.borrow()).iter_deref().read()) == 40));
    let j: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    let mut x: i32 = ((*{
        j.as_pointer().with_mut(|__it| {
            __it.iter_inc();
        });
        j.as_pointer()
    }
    .upgrade()
    .deref())
    .iter_deref()
    .read());
    assert!((x == 20));
    let k: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        (*{
            j.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            j.as_pointer()
        }
        .upgrade()
        .deref())
        .clone(),
    ));
    assert!((((*k.borrow()).iter_deref().read()) == 30));
    assert!((((*j.borrow()).iter_deref().read()) == 30));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
