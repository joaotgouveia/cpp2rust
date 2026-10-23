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
    ({ strcpy_1((carray.as_pointer() as Ptr<()>), (ccarray1).clone()) });
    let mut v: () = ((carray.as_pointer() as Ptr<()>) as Ptr<()>).to_any();
    let mut cv: () = (ccarray1).to_any();
    cc1 = ({ strchr_2((ccarray1).clone(), ('c' as ())) });
    c1 = ({ strchr_3((carray.as_pointer() as Ptr<()>), ('c' as ())) });
    cc1 = ({ strpbrk_4((ccarray1).clone(), (ccarray2).clone()) });
    c1 = ({ strpbrk_5((carray.as_pointer() as Ptr<()>), (ccarray2).clone()) });
    cc1 = ({ strrchr_6((ccarray1).clone(), ('c' as ())) });
    c1 = ({ strrchr_7((carray.as_pointer() as Ptr<()>), ('c' as ())) });
    cc1 = ({ strstr_8((ccarray1).clone(), (ccarray2).clone()) });
    c1 = ({
        let ___haystack: Ptr<()> = (carray.as_pointer() as Ptr<()>);
        let ___needle: Ptr<()> = (carray.as_pointer() as Ptr<()>);
        strstr_9(___haystack, ___needle)
    });
    cv = { ({ memchr_10((cv).clone(), ('a' as ()), 3_()) }) };
    v = { ({ memchr_11((v).clone(), ('a' as ()), 3_()) }) };
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
