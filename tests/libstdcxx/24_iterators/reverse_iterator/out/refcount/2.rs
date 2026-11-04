extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test02_0() {
    let it01: Value<std_reverse_iterator_int_ptr_> =
        Rc::new(RefCell::new(<std_reverse_iterator_int_ptr_>::default()));
    let it02: Value<std_reverse_iterator_int_ptr_> =
        Rc::new(RefCell::new(<std_reverse_iterator_int_ptr_>::default()));
    (*it01.borrow()) == (*it02.borrow());
    (*it01.borrow()) != (*it02.borrow());
    (*it01.borrow()) < (*it02.borrow());
    (*it01.borrow()) <= (*it02.borrow());
    (*it01.borrow()) > (*it02.borrow());
    (*it01.borrow()) >= (*it02.borrow());
    (*it01.borrow()).iter_distance(&({ (*it02.borrow()).clone() }).into()) as i64;
    (*it02.borrow()).iter_add(0_i64);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test02_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
