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
    let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -0_i32, -1_i32, -2_i32, -3_i32, -4_i32, -5_i32, -6_i32, -7_i32, -8_i32, -9_i32, -10_i32,
        -11_i32, -12_i32, -13_i32, -14_i32, -15_i32, -16_i32, -17_i32, -18_i32, -19_i32, -20_i32,
        -21_i32, -22_i32, -23_i32, -24_i32, -25_i32, -26_i32, -27_i32, -28_i32, -29_i32, -30_i32,
        -31_i32, -32_i32, -33_i32, -34_i32, -35_i32, -36_i32, -37_i32,
    ])));
    let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sz: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([2_usize, 4_usize, 3_usize])));
    let st: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([19_usize, 4_usize, 1_usize])));
    unimplemented!();
    assert!((unimplemented!() == 41_usize));
    assert!(((unimplemented!().read()) == 0));
    assert!(((unimplemented!().read()) == 1));
    assert!(((unimplemented!().read()) == 2));
    assert!(((unimplemented!().read()) == -3_i32));
    assert!(((unimplemented!().read()) == -4_i32));
    assert!(((unimplemented!().read()) == -5_i32));
    assert!(((unimplemented!().read()) == 6));
    assert!(((unimplemented!().read()) == -7_i32));
    assert!(((unimplemented!().read()) == -8_i32));
    assert!(((unimplemented!().read()) == -9_i32));
    assert!(((unimplemented!().read()) == 10));
    assert!(((unimplemented!().read()) == -11_i32));
    assert!(((unimplemented!().read()) == -12_i32));
    assert!(((unimplemented!().read()) == -13_i32));
    assert!(((unimplemented!().read()) == 14));
    assert!(((unimplemented!().read()) == -15_i32));
    assert!(((unimplemented!().read()) == -16_i32));
    assert!(((unimplemented!().read()) == -17_i32));
    assert!(((unimplemented!().read()) == 18));
    assert!(((unimplemented!().read()) == 19));
    assert!(((unimplemented!().read()) == 20));
    assert!(((unimplemented!().read()) == 21));
    assert!(((unimplemented!().read()) == -22_i32));
    assert!(((unimplemented!().read()) == -23_i32));
    assert!(((unimplemented!().read()) == -24_i32));
    assert!(((unimplemented!().read()) == 25));
    assert!(((unimplemented!().read()) == -26_i32));
    assert!(((unimplemented!().read()) == -27_i32));
    assert!(((unimplemented!().read()) == -28_i32));
    assert!(((unimplemented!().read()) == 29));
    assert!(((unimplemented!().read()) == -30_i32));
    assert!(((unimplemented!().read()) == -31_i32));
    assert!(((unimplemented!().read()) == -32_i32));
    assert!(((unimplemented!().read()) == 33));
    assert!(((unimplemented!().read()) == -34_i32));
    assert!(((unimplemented!().read()) == -35_i32));
    assert!(((unimplemented!().read()) == -36_i32));
    assert!(((unimplemented!().read()) == 37));
    assert!(((unimplemented!().read()) == 38));
    assert!(((unimplemented!().read()) == 39));
    assert!(((unimplemented!().read()) == 40));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
