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
    let mut fp: *mut ::libc::FILE = libc::fopen(path, Ptr::<()>::from_string_literal(b"wb"));
    unimplemented!();
    libc::fputs(Ptr::<()>::from_string_literal(b"hello"), fp);
    unimplemented!();
    let st: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_fstat_1() {
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_fstat_test.tmp");
    let mut fp: *mut ::libc::FILE = libc::fopen(path, Ptr::<()>::from_string_literal(b"wb"));
    unimplemented!();
    libc::fputs(Ptr::<()>::from_string_literal(b"hello world"), fp);
    libc::fflush(fp);
    let mut fd: () = libc::fileno(fp);
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
    let mut mtime: Ptr<::libc::time_t> =
        (unimplemented!().as_pointer()).reinterpret_cast::<::libc::time_t>();
    mtime.write((10 as ::libc::time_t));
    unimplemented!();
    let mut mtime_via_ptr: Ptr<::libc::time_t> =
        (unimplemented!().as_pointer()).reinterpret_cast::<::libc::time_t>();
    mtime_via_ptr.write((11 as ::libc::time_t));
    unimplemented!();
    let mut copy: ::libc::time_t = ((*unimplemented!().borrow()) as ::libc::time_t);
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
