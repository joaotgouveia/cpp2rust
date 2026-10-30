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
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            0_(),
            1_(),
            2_(),
            3_(),
            4_(),
            5_(),
            6_(),
            7_(),
            8_(),
            9_(),
            10_(),
            11_(),
            12_(),
            13_(),
            14_(),
            15_(),
            16_(),
            17_(),
            18_(),
            19_(),
            20_(),
            21_(),
            22_(),
            23_(),
            24_(),
            25_(),
            26_(),
            27_(),
            28_(),
            29_(),
            30_(),
            31_(),
            32_(),
            33_(),
            34_(),
            35_(),
            36_(),
            37_(),
            38_(),
            39_(),
            40_(),
        ])));
        let mut N1: () = (::std::mem::size_of::<[(); 41]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let s: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            3_(),
            4_(),
            5_(),
            7_(),
            8_(),
            9_(),
            11_(),
            12_(),
            13_(),
            15_(),
            16_(),
            17_(),
            22_(),
            23_(),
            24_(),
            26_(),
            27_(),
            28_(),
            30_(),
            31_(),
            32_(),
            34_(),
            35_(),
            36_(),
        ])));
        let mut S: () = (::std::mem::size_of::<[(); 24]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ia: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let raw_data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            0_(),
            1_(),
            2_(),
            3_(),
            4_(),
            5_(),
            6_(),
            7_(),
            8_(),
            9_(),
        ])));
        let idx_data: Value<Box<[()]>> =
            Rc::new(RefCell::new(Box::new([0_(), 2_(), 4_(), 6_(), 8_()])));
        let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let idx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
