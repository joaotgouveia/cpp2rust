extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test04_0() {
    let b1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let __s = libcc2rs::cout();
        let _ = Ptr::<i8>::from_string_literal(b"hello\n").with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test04_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
