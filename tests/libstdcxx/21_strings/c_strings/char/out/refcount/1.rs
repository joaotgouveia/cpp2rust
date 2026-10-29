extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let c: Value<()> = Rc::new(RefCell::new(('a' as ())));
    let cc: Value<()> = Rc::new(RefCell::new(('b' as ())));
    let mut c1: Ptr<()> = (c.as_pointer());
    let mut cc1: Ptr<()> = (cc.as_pointer());
    let mut ccarray1: Ptr<()> =
        Ptr::<()>::from_string_literal(b"san francisco roof garden inspectors");
    let mut ccarray2: Ptr<()> =
        Ptr::<()>::from_string_literal(b"san francisco sunny-day park inspectors");
    let carray: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..50).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    let mut v: () = ((carray.as_pointer() as Ptr<()>) as Ptr<()>).to_any();
    let mut cv: () = (ccarray1).to_any();
    cc1 = unimplemented!();
    c1 = unimplemented!();
    cc1 = unimplemented!();
    c1 = unimplemented!();
    cc1 = unimplemented!();
    c1 = unimplemented!();
    cc1 = unimplemented!();
    c1 = unimplemented!();
    cv = { unimplemented!() };
    v = { unimplemented!() };
    cc1 = { (cc1).clone() };
    c1 = { (c1).clone() };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
