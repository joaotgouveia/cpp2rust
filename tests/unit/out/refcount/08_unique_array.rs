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
    let g: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_0(2_()) })));
    (*g.borrow_mut())[0_()] = 11_();
    (*g.borrow_mut())[1_()] = 12_();
    let mut g_ptr: Ptr<()> = ({ (*g.borrow()).get() });
    elem!(g_ptr, 0_()).write(13_());
    elem!(g_ptr, 1_()).write(14_());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
