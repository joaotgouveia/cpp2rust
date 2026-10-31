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
        let __s = unimplemented!();
        libcc2rs::endl_refcount.call(__s)
    };
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    {
        let __s = {
            let __s = unimplemented!();
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
