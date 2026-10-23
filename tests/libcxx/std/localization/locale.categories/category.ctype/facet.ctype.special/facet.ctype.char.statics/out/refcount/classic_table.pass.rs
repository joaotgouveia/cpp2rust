extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    unimplemented!();
    unimplemented!();
    let mut p: Ptr<()> = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        let mut expect_cntrl: () = ((i < 32_()) || (126_() < i));
        let mut expect_print: () = ((32_() <= i) && (i <= 126_()));
        let mut expect_space: () = ((9_() <= i) && (i <= 13_())) || (i == (' ' as ()));
        let mut expect_blank: () = ((i == ('\t' as ())) || (i == (' ' as ())));
        let mut expect_upper: () = ((('A' as ()) <= i) && (i <= ('Z' as ())));
        let mut expect_lower: () = ((('a' as ()) <= i) && (i <= ('z' as ())));
        let mut expect_alpha: () = (expect_upper) || (expect_lower);
        let mut expect_digit: () = ((('0' as ()) <= i) && (i <= ('9' as ())));
        let mut expect_xdigit: () = (((('A' as ()) <= i) && (i <= ('F' as ())))
            || ((('a' as ()) <= i) && (i <= ('f' as ()))))
            || (expect_digit);
        let mut expect_punct: () = ((((33_() <= i) && (i <= 47_()))
            || ((58_() <= i) && (i <= 64_())))
            || ((91_() <= i) && (i <= 96_())))
            || ((123_() <= i) && (i <= 126_()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
