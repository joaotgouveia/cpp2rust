extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<(Value<Ptr<i8>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Ptr::<i8>::from_string_literal(b"converted"))),
        Rc::new(RefCell::new(99)),
    )));
    assert!(
        ({
            ({
                let mut __it1 = (*(*p.borrow()).0.borrow()).to_c_string_iterator();
                let mut __it2 = Ptr::<i8>::from_string_literal(b"converted").to_c_string_iterator();
                loop {
                    let __c1 = __it1.next();
                    let __c2 = __it2.next();
                    if __c1 != __c2 {
                        break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                    }
                    if __c1.is_none() {
                        break 0;
                    }
                }
            } == 0)
                && ((*(*p.borrow()).1.borrow()) == 99)
        })
    );
    let s: Value<(Value<Vec<i8>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"k").to_c_bytes();
            __bytes.push(0);
            __bytes
        })),
        Rc::new(RefCell::new(2)),
    )));
    assert!(
        ({
            (Ptr::<i8>::from_string_literal(b"k").with_c_str(|__s| {
                (*(*s.borrow()).0.borrow())[..(*(*s.borrow()).0.borrow()).len().saturating_sub(1)]
                    == *__s
            })) && ((*(*s.borrow()).1.borrow()) == 2)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
