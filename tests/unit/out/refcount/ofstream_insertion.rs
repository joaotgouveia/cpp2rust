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
    {
        let out: Value<::std::fs::File> = Rc::new(RefCell::new(
            ::std::fs::File::create(
                Ptr::<i8>::from_string_literal(b"./ofstream_insertion.txt").to_string(),
            )
            .expect("Failed to open file"),
        ));
        write!(out.as_pointer(), "{:}", 42,);
        write!(out.as_pointer(), " items",);
    }
    let mut f: Ptr<CFile> = match CFile::open(
        &Ptr::<i8>::from_string_literal(b"./ofstream_insertion.txt").to_rust_string(),
        &Ptr::<i8>::from_string_literal(b"rb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        0_i8,
    ])));
    let mut n: usize = libcc2rs::fread_refcount(
        ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
        1_usize,
        ::std::mem::size_of::<[i8; 16]>(),
        (f).clone(),
    );
    {
        let __r = f.with(|__f| __f.close());
        f.delete();
        __r
    };
    assert!((n == 8_usize));
    assert!(
        ({
            (((((*buf.borrow())[(0) as usize] as i32) == (('4' as i8) as i32))
                && (((*buf.borrow())[(1) as usize] as i32) == (('2' as i8) as i32)))
                && (((*buf.borrow())[(2) as usize] as i32) == ((' ' as i8) as i32)))
                && (((*buf.borrow())[(7) as usize] as i32) == (('s' as i8) as i32))
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
