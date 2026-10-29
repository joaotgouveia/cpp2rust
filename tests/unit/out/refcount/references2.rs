extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn change_0(p: Ptr<std_unique_ptr_int__std_default_delete_int__>) {
    let q: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(7_()));
            make_unique_1(___args.as_pointer())
        }),
    ));
    p.write({ (*q.borrow()).clone() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(5_()));
            make_unique_1(___args.as_pointer())
        }),
    ));
    ({ change_0(a.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
