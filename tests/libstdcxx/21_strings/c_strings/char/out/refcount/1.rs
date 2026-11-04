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
    cc1 = {
        let __t = (('c' as i8) as i32) as i8;
        match ccarray1.to_c_string_iterator().position(|__c| __c == __t) {
            Some(__i) => ccarray1.offset(__i),
            None => {
                if __t == 0 {
                    ccarray1.offset(ccarray1.to_c_string_iterator().count())
                } else {
                    Ptr::null()
                }
            }
        }
    };
    c1 = ({ libcc2rs::strchr_refcount((carray.as_pointer() as Ptr<i8>), (('c' as i8) as i32)) });
    cc1 = match ccarray1
        .to_c_string_iterator()
        .position(|__c| ccarray2.to_c_string_iterator().any(|__r| __r == __c))
    {
        Some(__i) => ccarray1.offset(__i),
        None => Ptr::null(),
    };
    c1 = match ({ (carray.as_pointer() as Ptr<i8>) },) {
        (mut __tmp_0,) => {
            match __tmp_0
                .to_c_string_iterator()
                .position(|__c| ccarray2.to_c_string_iterator().any(|__r| __r == __c))
            {
                Some(__i) => __tmp_0.offset(__i),
                None => Ptr::null(),
            }
        }
    };
    cc1 = {
        let __t = (('c' as i8) as i32) as i8;
        match ccarray1
            .to_c_string_iterator()
            .enumerate()
            .filter(|__e| __e.1 == __t)
            .last()
        {
            Some((__i, _)) => ccarray1.offset(__i),
            None => {
                if __t == 0 {
                    ccarray1.offset(ccarray1.to_c_string_iterator().count())
                } else {
                    Ptr::null()
                }
            }
        }
    };
    c1 = match ({ (carray.as_pointer() as Ptr<i8>) },) {
        (mut __tmp_1,) => {
            let __t = (('c' as i8) as i32) as i8;
            match __tmp_1
                .to_c_string_iterator()
                .enumerate()
                .filter(|__e| __e.1 == __t)
                .last()
            {
                Some((__i, _)) => __tmp_1.offset(__i),
                None => {
                    if __t == 0 {
                        __tmp_1.offset(__tmp_1.to_c_string_iterator().count())
                    } else {
                        Ptr::null()
                    }
                }
            }
        }
    };
    cc1 = {
        let mut __p = (ccarray1).clone();
        loop {
            let mut __h = __p.to_c_string_iterator();
            if ccarray2
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
    c1 = match ({ (carray.as_pointer() as Ptr<i8>) },) {
        (mut __tmp_2,) => {
            let mut __p = (carray.as_pointer() as Ptr<i8>);
            loop {
                let mut __h = __p.to_c_string_iterator();
                if __tmp_2
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
        }
    };
    cv = {
        {
            let mut __p = cv.reinterpret_cast::<u8>();
            let mut __i: usize = 0;
            loop {
                if __i == 3_usize {
                    break Ptr::<u8>::null().to_any();
                }
                if __p.read() == (('a' as i8) as i32) as u8 {
                    break __p.to_any();
                }
                __p += 1;
                __i += 1;
            }
        }
    };
    v = { ({ libcc2rs::memchr_refcount((v).clone(), (('a' as i8) as i32), 3_usize) }) };
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
