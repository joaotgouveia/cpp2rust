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
    let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    ])));
    let mut N1: usize = (::std::mem::size_of::<[i32; 41]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize));
    let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ])));
    let N2: Value<usize> = Rc::new(RefCell::new(
        (::std::mem::size_of::<[i32; 24]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)),
    ));
    let s: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        3_usize, 4_usize, 5_usize, 7_usize, 8_usize, 9_usize, 11_usize, 12_usize, 13_usize,
        15_usize, 16_usize, 17_usize, 22_usize, 23_usize, 24_usize, 26_usize, 27_usize, 28_usize,
        30_usize, 31_usize, 32_usize, 34_usize, 35_usize, 36_usize,
    ])));
    let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ia: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!((unimplemented!() == 41_usize));
    assert!(((unimplemented!().read()) == 0));
    assert!(((unimplemented!().read()) == 1));
    assert!(((unimplemented!().read()) == 2));
    assert!(((unimplemented!().read()) == 2));
    assert!(((unimplemented!().read()) == 2));
    assert!(((unimplemented!().read()) == 2));
    assert!(((unimplemented!().read()) == 6));
    assert!(((unimplemented!().read()) == 3));
    assert!(((unimplemented!().read()) == 3));
    assert!(((unimplemented!().read()) == 3));
    assert!(((unimplemented!().read()) == 10));
    assert!(((unimplemented!().read()) == 4));
    assert!(((unimplemented!().read()) == 4));
    assert!(((unimplemented!().read()) == 4));
    assert!(((unimplemented!().read()) == 14));
    assert!(((unimplemented!().read()) == 5));
    assert!(((unimplemented!().read()) == 5));
    assert!(((unimplemented!().read()) == 5));
    assert!(((unimplemented!().read()) == 18));
    assert!(((unimplemented!().read()) == 19));
    assert!(((unimplemented!().read()) == 20));
    assert!(((unimplemented!().read()) == 21));
    assert!(((unimplemented!().read()) == 9));
    assert!(((unimplemented!().read()) == 9));
    assert!(((unimplemented!().read()) == 9));
    assert!(((unimplemented!().read()) == 25));
    assert!(((unimplemented!().read()) == 10));
    assert!(((unimplemented!().read()) == 10));
    assert!(((unimplemented!().read()) == 10));
    assert!(((unimplemented!().read()) == 29));
    assert!(((unimplemented!().read()) == 11));
    assert!(((unimplemented!().read()) == 11));
    assert!(((unimplemented!().read()) == 11));
    assert!(((unimplemented!().read()) == 33));
    assert!(((unimplemented!().read()) == 12));
    assert!(((unimplemented!().read()) == 12));
    assert!(((unimplemented!().read()) == 12));
    assert!(((unimplemented!().read()) == 37));
    assert!(((unimplemented!().read()) == 38));
    assert!(((unimplemented!().read()) == 39));
    assert!(((unimplemented!().read()) == 40));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
