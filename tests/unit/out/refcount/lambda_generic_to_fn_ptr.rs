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
    let negate: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: ()| -> () {
            return -x;
        },
        |mut x: ()| -> () {
            return -x;
        }
    )));
    let mut fi: FnPtr<fn(()) -> ()> = (*negate.borrow()).clone().spec::<fn(()) -> ()>(0);
    let mut fd: FnPtr<fn(()) -> ()> = (*negate.borrow()).clone().spec::<fn(()) -> ()>(1);
    unimplemented!();
    unimplemented!();
    let square: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: ()| -> () {
            return (x * x);
        },
        |mut x: ()| -> () {
            return (x * x);
        }
    )));
    let mut si: FnPtr<fn(()) -> ()> = (*square.borrow()).clone().spec::<fn(()) -> ()>(0);
    let mut sd: FnPtr<fn(()) -> ()> = (*square.borrow()).clone().spec::<fn(()) -> ()>(1);
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
