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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let mut b: i32 = 42;
    let r: Ptr<i32> = ({
        (*a.borrow_mut()) = b;
        a.as_pointer()
    });
    assert!(((*a.borrow()) == 42));
    assert!(({ (r).clone() } == { (a.as_pointer()) }));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((arr.as_pointer() as Ptr<i32>)));
    let pr: Ptr<Ptr<i32>> = ({
        (*p.borrow_mut()) += 3;
        p.as_pointer()
    });
    assert!((((pr.read()).read()) == 4));
    assert!(({ (pr).clone() } == { (p.as_pointer()) }));
    ({
        (*a.borrow_mut()) = b;
        a.as_pointer()
    })
    .write(5);
    assert!(((*a.borrow()) == 5));
    assert!((b == 42));
    r.write(7);
    assert!(((*a.borrow()) == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
