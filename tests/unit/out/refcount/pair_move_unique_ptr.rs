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
    let u: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Ptr::alloc(3).to_owned_opt()));
    let p: Value<(Value<Option<Value<i32>>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(CppMove::cpp_move(u.as_pointer()))),
        Rc::new(RefCell::new(1)),
    )));
    assert!(((*u.borrow()).as_pointer()).is_null());
    assert!((((*(*p.borrow()).0.borrow()).as_pointer().read()) == 3));
    let source: Value<(Value<i32>, Value<Option<Value<i32>>>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(0)),
        Rc::new(RefCell::new(Ptr::alloc(81).to_owned_opt())),
    )));
    let mut original: Ptr<i32> = (*(*source.borrow()).1.borrow()).as_pointer();
    let converted: Value<(Value<i64>, Value<Option<Value<i32>>>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            (((*source.borrow()).0.as_pointer().read()) as i64),
        )),
        Rc::new(RefCell::new(<Option<Value<i32>> as CppFrom<
            Option<Value<i32>>,
        >>::cpp_from_move(
            (*source.borrow()).1.as_pointer()
        ))),
    )));
    assert!(((*(*converted.borrow()).0.borrow()) == 0_i64));
    assert!(({ (*(*converted.borrow()).1.borrow()).as_pointer() } == { (original).clone() }));
    assert!((((*(*converted.borrow()).1.borrow()).as_pointer().read()) == 81));
    assert!(((*(*source.borrow()).1.borrow()).as_pointer()).is_null());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
