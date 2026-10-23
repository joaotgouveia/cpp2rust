extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test02_0() {
    let mut ccarray1: Ptr<()> =
        Ptr::<()>::from_string_literal(b"san francisco roof garden inspectors");
    let mut ccarray2: Ptr<()> =
        Ptr::<()>::from_string_literal(b"san francisco sunny-day park inspectors");
    let carray: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..50).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({ strcpy_1((carray.as_pointer() as Ptr<()>), (ccarray1).clone()) });
    let mut cv: () = (ccarray1).to_any();
    let mut cv1: () = unimplemented!();
    let mut cc: Ptr<()> = Ptr::<()>::null();
    let mut c: Ptr<()> = Ptr::<()>::null();
    cv1 = ({ memchr_2((cv).clone(), ('/' as ()), 3_()) });
    cc = ({ strchr_3((ccarray1).clone(), ('/' as ())) });
    cc = ({ strrchr_4((ccarray1).clone(), ('c' as ())) });
    cc = ({ strpbrk_5((ccarray1).clone(), (ccarray2).clone()) });
    c = ({
        let ___haystack: Ptr<()> = (carray.as_pointer() as Ptr<()>);
        let ___needle: Ptr<()> = (carray.as_pointer() as Ptr<()>);
        strstr_6(___haystack, ___needle)
    });
    cv1 = { (cv1).clone() };
    cc = { (cc).clone() };
    c = { (c).clone() };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test02_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
