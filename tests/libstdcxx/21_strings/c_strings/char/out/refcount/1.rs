extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let c: Value<i8> = Rc::new(RefCell::new(('a' as i8)));
    let cc: Value<i8> = Rc::new(RefCell::new(('b' as i8)));
    let mut c1: Ptr<i8> = (c.as_pointer());
    let mut cc1: Ptr<i8> = (cc.as_pointer());
    let mut ccarray1: Ptr<i8> =
        Ptr::<i8>::from_string_literal(b"san francisco roof garden inspectors");
    let mut ccarray2: Ptr<i8> =
        Ptr::<i8>::from_string_literal(b"san francisco sunny-day park inspectors");
    let carray: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..50).map(|_| 0_i8).collect::<Box<[i8]>>()));
    ({ strcpy_1((carray.as_pointer() as Ptr<i8>), (ccarray1).clone()) });
    let mut v: AnyPtr = ((carray.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
    let mut cv: AnyPtr = (ccarray1).to_any();
    cc1 = ({ strchr_2((ccarray1).clone(), (('c' as i8) as i32)) });
    c1 = ({ strchr_3((carray.as_pointer() as Ptr<i8>), (('c' as i8) as i32)) });
    cc1 = ({ strpbrk_4((ccarray1).clone(), (ccarray2).clone()) });
    c1 = ({ strpbrk_5((carray.as_pointer() as Ptr<i8>), (ccarray2).clone()) });
    cc1 = ({ strrchr_6((ccarray1).clone(), (('c' as i8) as i32)) });
    c1 = ({ strrchr_7((carray.as_pointer() as Ptr<i8>), (('c' as i8) as i32)) });
    cc1 = ({ strstr_8((ccarray1).clone(), (ccarray2).clone()) });
    c1 = ({
        let ___haystack: Ptr<i8> = (carray.as_pointer() as Ptr<i8>);
        let ___needle: Ptr<i8> = (carray.as_pointer() as Ptr<i8>);
        strstr_9(___haystack, ___needle)
    });
    cv = { ({ memchr_10((cv).clone(), (('a' as i8) as i32), 3_u64) }) };
    v = { ({ memchr_11((v).clone(), (('a' as i8) as i32), 3_u64) }) };
    cc1 = { (cc1).clone() };
    c1 = { (c1).clone() };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
