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
    let v: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::new()));
    let s: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"hi").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    {
        let a0_clone = (*s.borrow()).clone();
        (*v.borrow_mut()).push(a0_clone)
    };
    {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"changed").to_c_bytes();
        __bytes.push(0);
        s.as_pointer().write(__bytes);
        s.as_pointer()
    };
    assert!(Ptr::<i8>::from_string_literal(b"hi").with_c_str(|__s| {
        (*v.decay().offset(0_usize).upgrade().deref())[..(*v
            .decay()
            .offset(0_usize)
            .upgrade()
            .deref())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(
        Ptr::<i8>::from_string_literal(b"changed")
            .with_c_str(|__s| (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s)
    );
    assert!((((*s.borrow()).len() - 1) == 7_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
