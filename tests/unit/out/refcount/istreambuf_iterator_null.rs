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
    let mut first: Ptr<::std::fs::File> = Ptr::<::std::fs::File>::null();
    let mut last: Ptr<::std::fs::File> = Ptr::<::std::fs::File>::null();
    let mut s: Vec<i8> = {
        let mut __bytes: Vec<u8> = Vec::new();
        if !(first).clone().is_null() {
            (first)
                .clone()
                .with_mut(|__f: &mut std::fs::File| std::io::Read::read_to_end(__f, &mut __bytes))
                .expect("couldn't read the file");
        }
        __bytes.push(0);
        CChar::from_byte_vec(__bytes)
    };
    assert!(s.len() <= 1);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
