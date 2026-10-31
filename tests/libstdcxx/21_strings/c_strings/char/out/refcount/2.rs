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
    unimplemented!();
    let mut cv: AnyPtr = (ccarray1).to_any();
    let mut cv1: AnyPtr = AnyPtr::default();
    let mut cc: Ptr<i8> = Ptr::<i8>::null();
    let mut c: Ptr<i8> = Ptr::<i8>::null();
    cv1 = {
        let mut __p = cv.reinterpret_cast::<u8>();
        let mut __i: usize = 0;
        loop {
            if __i == 3_usize {
                break Ptr::<u8>::null().to_any();
            }
            if __p.read() == (('/' as i8) as i32) as u8 {
                break __p.to_any();
            }
            __p += 1;
            __i += 1;
        }
    };
    cc = {
        let __s = (ccarray1).clone();
        let __t = (('/' as i8) as i32) as i8;
        match __s.to_c_string_iterator().position(|__c| __c == __t) {
            Some(__i) => __s.offset(__i),
            None => {
                if __t == 0 {
                    __s.offset(__s.to_c_string_iterator().count())
                } else {
                    Ptr::null()
                }
            }
        }
    };
    cc = {
        let __s = (ccarray1).clone();
        let __t = (('c' as i8) as i32) as i8;
        match __s
            .to_c_string_iterator()
            .enumerate()
            .filter(|__e| __e.1 == __t)
            .last()
        {
            Some((__i, _)) => __s.offset(__i),
            None => {
                if __t == 0 {
                    __s.offset(__s.to_c_string_iterator().count())
                } else {
                    Ptr::null()
                }
            }
        }
    };
    cc = {
        let __s = (ccarray1).clone();
        let __set = (ccarray2).clone();
        match __s
            .to_c_string_iterator()
            .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
        {
            Some(__i) => __s.offset(__i),
            None => Ptr::null(),
        }
    };
    c = {
        let __needle = (carray.as_pointer() as Ptr<i8>);
        let mut __p = (carray.as_pointer() as Ptr<i8>);
        loop {
            let mut __h = __p.to_c_string_iterator();
            if __needle
                .to_c_string_iterator()
                .all(|__c| __h.next() == Some(__c))
            {
                break __p;
            }
            if __p.read() == 0 {
                break Ptr::null();
            }
            __p += 1;
        }
    };
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
