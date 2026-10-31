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
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ])));
        let N1: Value<usize> = Rc::new(RefCell::new(
            (::std::mem::size_of::<[i32; 16]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)),
        ));
        let b1: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([
            true, false, false, true, true, false, false, true, false, false, false, true, false,
            false, false, false,
        ])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            -1_i32, -2_i32, -3_i32, -4_i32, -5_i32, -6_i32, -7_i32, -8_i32,
        ])));
        let N2: Value<usize> = Rc::new(RefCell::new(
            (::std::mem::size_of::<[i32; 8]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)),
        ));
        let b2: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([
            true, false, true, true, false, false, true, true,
        ])));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vb1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vb2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!() == 16_usize));
        assert!(((unimplemented!().read()) == -1_i32));
        assert!(((unimplemented!().read()) == 1));
        assert!(((unimplemented!().read()) == 2));
        assert!(((unimplemented!().read()) == -3_i32));
        assert!(((unimplemented!().read()) == -4_i32));
        assert!(((unimplemented!().read()) == 5));
        assert!(((unimplemented!().read()) == 6));
        assert!(((unimplemented!().read()) == -7_i32));
        assert!(((unimplemented!().read()) == 8));
        assert!(((unimplemented!().read()) == 9));
        assert!(((unimplemented!().read()) == 10));
        assert!(((unimplemented!().read()) == -8_i32));
        assert!(((unimplemented!().read()) == 12));
        assert!(((unimplemented!().read()) == 13));
        assert!(((unimplemented!().read()) == 14));
        assert!(((unimplemented!().read()) == 15));
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 4, 5])));
        let b1: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([true, false, true])));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vb1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Ptr<()> = (unimplemented!());
        assert!(({ (r).clone() } == { (m1.as_pointer()) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
