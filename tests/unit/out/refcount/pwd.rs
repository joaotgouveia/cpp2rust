extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_getpwuid_0() {
    let mut pw: Ptr<passwd> = ({ getpwuid_1(unimplemented!()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    println!("{}", pw.with(|__s| __s.pw_name.clone()));
}
pub fn test_getpwuid_missing_2() {
    unimplemented!().write(0_());
    let mut pw: Ptr<passwd> = ({ getpwuid_1(2147483646_()) });
    unimplemented!();
    unimplemented!();
}
pub fn test_getpwuid_r_3() {
    let pw: Value<passwd> = <Value<passwd>>::default();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4096).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let result: Value<Ptr<passwd>> = Rc::new(RefCell::new(Ptr::<passwd>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut pw2: Ptr<passwd> = ({ getpwuid_1(unimplemented!()) });
    unimplemented!();
    unimplemented!();
    println!("{}", { (*pw.borrow()).pw_name.clone() });
}
pub fn test_getpwuid_r_erange_4() {
    let pw: Value<passwd> = <Value<passwd>>::default();
    let tiny: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..1).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let result: Value<Ptr<passwd>> = Rc::new(RefCell::new(Ptr::<passwd>::null()));
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_getpwuid_0() });
    ({ test_getpwuid_missing_2() });
    ({ test_getpwuid_r_3() });
    ({ test_getpwuid_r_erange_4() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
