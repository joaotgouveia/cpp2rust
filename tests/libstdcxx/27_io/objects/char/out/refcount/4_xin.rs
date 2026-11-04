extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test10_0() {
    {
        let __s = libcc2rs::cout();
        let _ = Ptr::<i8>::from_string_literal(b"Press ENTER once\n").with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
    ({ (*cin_1.with(Value::clone).borrow()).ignore_30(1_i64) });
    {
        let __s = {
            let __s = {
                let __s = libcc2rs::cout();
                let _ = Ptr::<i8>::from_string_literal(b"_M_gcount: ").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            write_int_refcount(
                &__s,
                ({ ({ (*cin_1.with(Value::clone).borrow()).gcount() }) }),
            );
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test10_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
