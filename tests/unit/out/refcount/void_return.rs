extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f1_0(first: (), last: ()) {
    let first: Value<()> = Rc::new(RefCell::new(first));
    let last: Value<()> = Rc::new(RefCell::new(last));
    unimplemented!();
    return;
}
pub fn f1_1(mut first: Ptr<()>, mut last: Ptr<()>) {
    unimplemented!();
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut v: () = unimplemented!();
    ({
        let _first: () = unimplemented!();
        let _last: () = unimplemented!();
        f1_0(_first, _last)
    });
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    ({
        let _first: Ptr<()> = (a.as_pointer() as Ptr<()>);
        let _last: Ptr<()> = (a.as_pointer() as Ptr<()>).offset((2_()) as isize);
        f1_1(_first, _last)
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
