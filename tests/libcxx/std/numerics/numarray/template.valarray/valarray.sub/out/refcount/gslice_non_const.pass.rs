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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        ])));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sz: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([2_usize, 4_usize, 3_usize])));
        let st: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([19_usize, 4_usize, 1_usize])));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 24_usize));
        assert!(((unimplemented!().read()) == 3));
        assert!(((unimplemented!().read()) == 4));
        assert!(((unimplemented!().read()) == 5));
        assert!(((unimplemented!().read()) == 7));
        assert!(((unimplemented!().read()) == 8));
        assert!(((unimplemented!().read()) == 9));
        assert!(((unimplemented!().read()) == 11));
        assert!(((unimplemented!().read()) == 12));
        assert!(((unimplemented!().read()) == 13));
        assert!(((unimplemented!().read()) == 15));
        assert!(((unimplemented!().read()) == 16));
        assert!(((unimplemented!().read()) == 17));
        assert!(((unimplemented!().read()) == 22));
        assert!(((unimplemented!().read()) == 23));
        assert!(((unimplemented!().read()) == 24));
        assert!(((unimplemented!().read()) == 26));
        assert!(((unimplemented!().read()) == 27));
        assert!(((unimplemented!().read()) == 28));
        assert!(((unimplemented!().read()) == 30));
        assert!(((unimplemented!().read()) == 31));
        assert!(((unimplemented!().read()) == 32));
        assert!(((unimplemented!().read()) == 34));
        assert!(((unimplemented!().read()) == 35));
        assert!(((unimplemented!().read()) == 36));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        ])));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sz: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([2_usize, 4_usize, 3_usize])));
        let st: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([19_usize, 4_usize, 1_usize])));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 24_usize));
        assert!(((unimplemented!().read()) == 3));
        assert!(((unimplemented!().read()) == 4));
        assert!(((unimplemented!().read()) == 5));
        assert!(((unimplemented!().read()) == 7));
        assert!(((unimplemented!().read()) == 8));
        assert!(((unimplemented!().read()) == 9));
        assert!(((unimplemented!().read()) == 11));
        assert!(((unimplemented!().read()) == 12));
        assert!(((unimplemented!().read()) == 13));
        assert!(((unimplemented!().read()) == 15));
        assert!(((unimplemented!().read()) == 16));
        assert!(((unimplemented!().read()) == 17));
        assert!(((unimplemented!().read()) == 22));
        assert!(((unimplemented!().read()) == 23));
        assert!(((unimplemented!().read()) == 24));
        assert!(((unimplemented!().read()) == 26));
        assert!(((unimplemented!().read()) == 27));
        assert!(((unimplemented!().read()) == 28));
        assert!(((unimplemented!().read()) == 30));
        assert!(((unimplemented!().read()) == 31));
        assert!(((unimplemented!().read()) == 32));
        assert!(((unimplemented!().read()) == 34));
        assert!(((unimplemented!().read()) == 35));
        assert!(((unimplemented!().read()) == 36));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
