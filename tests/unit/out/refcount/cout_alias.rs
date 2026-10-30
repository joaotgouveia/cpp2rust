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
    let mut cond: bool = true;
    let mut os1: Ptr<::std::fs::File> = if cond {
        (libcc2rs::cout())
    } else {
        (libcc2rs::cerr())
    };
    {
        let __s = (os1).clone();
        let _ = Ptr::<i8>::from_string_literal(b"hello\n").with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
    let os2: Ptr<::std::fs::File> = if cond {
        (libcc2rs::cout())
    } else {
        (libcc2rs::cerr())
    };
    {
        let __s = (os2).clone();
        let _ = Ptr::<i8>::from_string_literal(b"hello\n").with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
