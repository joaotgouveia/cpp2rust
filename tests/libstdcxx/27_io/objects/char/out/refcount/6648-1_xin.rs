extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test08_0() {
    let buff: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..2048).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let __s = libcc2rs::cout();
        let _ = Ptr::<i8>::from_string_literal(b"Enter name: ").with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
    ({ (*cin_1.with(Value::clone).borrow()).getline_28((buff.as_pointer() as Ptr<i8>), 2048_i64) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test08_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
