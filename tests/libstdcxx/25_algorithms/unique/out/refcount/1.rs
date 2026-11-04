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
    let menge: Value<std_list_int__std_allocator_int__> =
        Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
    ({
        let ___first: std__List_iterator_int_ = ({ (*menge.borrow()).begin_24() });
        let ___last: std__List_iterator_int_ = ({ (*menge.borrow()).end_26() });
        unique_0(___first, ___last)
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
