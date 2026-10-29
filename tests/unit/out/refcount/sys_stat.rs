extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_stat_0() {
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_stat_test.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let st: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_fstat_1() {
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_fstat_test.tmp");
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fd: () = unimplemented!();
    let st: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_st_mtime_access_2() {
    let st: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut sp: Ptr<()> = (st.as_pointer());
    (*unimplemented!().borrow_mut()) = 5_();
    unimplemented!();
    (*unimplemented!().borrow_mut()) = 6_();
    unimplemented!();
    (*unimplemented!().borrow_mut()) += 2_();
    unimplemented!();
    (*unimplemented!().borrow_mut()) -= 1_();
    unimplemented!();
    (*unimplemented!().borrow_mut()).postfix_inc();
    (*unimplemented!().borrow_mut()).prefix_inc();
    unimplemented!();
    let mut mtime: Ptr<()> = (unimplemented!().as_pointer());
    mtime.write(10_());
    unimplemented!();
    let mut mtime_via_ptr: Ptr<()> = (unimplemented!().as_pointer());
    mtime_via_ptr.write(11_());
    unimplemented!();
    let mut copy: () = (*unimplemented!().borrow());
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_stat_0() });
    ({ test_fstat_1() });
    ({ test_st_mtime_access_2() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
