extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test02_0() {
    let mut ccarray1: Ptr<i8> =
        Ptr::<i8>::from_string_literal(b"san francisco roof garden inspectors");
    let mut ccarray2: Ptr<i8> =
        Ptr::<i8>::from_string_literal(b"san francisco sunny-day park inspectors");
    let carray: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..50).map(|_| 0_i8).collect::<Box<[i8]>>()));
    ({ strcpy_1((carray.as_pointer() as Ptr<i8>), (ccarray1).clone()) });
    let mut cv: AnyPtr = (ccarray1).to_any();
    let mut cv1: AnyPtr = AnyPtr::default();
    let mut cc: Ptr<i8> = Ptr::<i8>::null();
    let mut c: Ptr<i8> = Ptr::<i8>::null();
    cv1 = ({ memchr_2((cv).clone(), (('/' as i8) as i32), 3_u64) });
    cc = ({ strchr_3((ccarray1).clone(), (('/' as i8) as i32)) });
    cc = ({ strrchr_4((ccarray1).clone(), (('c' as i8) as i32)) });
    cc = ({ strpbrk_5((ccarray1).clone(), (ccarray2).clone()) });
    c = ({
        let ___haystack: Ptr<i8> = (carray.as_pointer() as Ptr<i8>);
        let ___needle: Ptr<i8> = (carray.as_pointer() as Ptr<i8>);
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
fn main_0() -> i32 {
    ({ test02_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
