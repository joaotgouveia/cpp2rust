extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test05_0() {
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    let _ = Ptr::<i8>::from_string_literal(b"hello").with_c_str(|__cs| {
                        __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                    });
                    __s
                };
                let _ = __s.write_all(&[(' ' as i8) as u8]);
                __s
            };
            let _ = Ptr::<i8>::from_string_literal(b"world").with_c_str(|__cs| {
                __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
            });
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
    {
        let __s = libcc2rs::cout();
        let _ = Ptr::<i8>::from_string_literal(b"Enter your name: ").with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
    let mut s: Vec<i8> = vec![i8::default()];
    unimplemented!();
    {
        let __s = {
            let __s = {
                let __s = libcc2rs::cout();
                let _ = Ptr::<i8>::from_string_literal(b"hello ").with_c_str(|__cs| {
                    __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
                });
                __s
            };
            let mut __bytes = Vec::new();
            for &__c in &s[..s.len().saturating_sub(1)] {
                __c.push_stream_bytes(&mut __bytes);
            }
            let _ = __s.write_all(&__bytes);
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
    ({ test05_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
