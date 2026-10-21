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
fn main_0() -> () {
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let __decomp_0: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 3, _>(
            |__i: usize| (*arr.borrow())[(__i) as usize],
        ))));
    (*__decomp_0.borrow_mut())[(unimplemented!()) as usize] = 10_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_1: Ptr<()> = arr.as_pointer();
    elem!((__decomp_1), unimplemented!()).write(7_());
    {
        let __rhs = (elem!((__decomp_1), 1_()).read());
        elem!((__decomp_1), 2_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
