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
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_(), 10_()];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_(), 10_(), 0_()];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [10_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_()];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_()];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 1_()];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_(), 0_()];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut N1: () = 0_();
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [
            8_(),
            10_(),
            12_(),
            14_(),
            16_(),
            18_(),
            20_(),
            0_(),
            0_(),
            0_(),
        ];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let mut a2: [(); 10] = [
            0_(),
            0_(),
            0_(),
            2_(),
            4_(),
            6_(),
            8_(),
            10_(),
            12_(),
            14_(),
        ];
        let mut N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N1) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
