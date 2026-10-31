extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test06_0() {
    let mut i: i32 = 0_i32;
    unimplemented!();
    if !(unimplemented!()) {
        {
            let __s = libcc2rs::cerr();
            libcc2rs::endl_refcount.call(__s)
        };
        {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cerr();
                    let _ = Ptr::<i8>::from_string_literal(b"i == ").with_c_str(|__cs| {
                        __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                    });
                    __s
                };
                write_int_refcount(&__s, i);
                __s
            };
            libcc2rs::endl_refcount.call(__s)
        };
        {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cerr();
                    let _ =
                        Ptr::<i8>::from_string_literal(b"cin.rdstate() == ").with_c_str(|__cs| {
                            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                        });
                    __s
                };
                write_int_refcount(&__s, (unimplemented!() as i32));
                __s
            };
            libcc2rs::endl_refcount.call(__s)
        };
        {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cerr();
                    let _ = Ptr::<i8>::from_string_literal(b"cin.bad() == ").with_c_str(|__cs| {
                        __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                    });
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            libcc2rs::endl_refcount.call(__s)
        };
        {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cerr();
                    let _ = Ptr::<i8>::from_string_literal(b"cin.fail() == ").with_c_str(|__cs| {
                        __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                    });
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            libcc2rs::endl_refcount.call(__s)
        };
        {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cerr();
                    let _ = Ptr::<i8>::from_string_literal(b"cin.eof() == ").with_c_str(|__cs| {
                        __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                    });
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            libcc2rs::endl_refcount.call(__s)
        };
    } else {
        {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cerr();
                    let _ = Ptr::<i8>::from_string_literal(b"i == ").with_c_str(|__cs| {
                        __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                    });
                    __s
                };
                write_int_refcount(&__s, i);
                __s
            };
            libcc2rs::endl_refcount.call(__s)
        };
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test06_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
