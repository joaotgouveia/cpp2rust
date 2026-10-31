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
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        let mut N: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        let mut i: usize = 0_usize;
        'loop_: while (i < unimplemented!()) {
            assert!(((unimplemented!().read()) == (unimplemented!().read())));
            i.prefix_inc();
        }
    }
    {
        let a: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
            1_f64, 2.5_f64, 3_f64, 4.25_f64, 5_f64,
        ])));
        let mut N: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize)))
            as u32);
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        let mut i: usize = 0_usize;
        'loop_: while (i < unimplemented!()) {
            assert!(((unimplemented!().read()) == (unimplemented!().read())));
            i.prefix_inc();
        }
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let mut N: u32 = (((80usize as usize).wrapping_div((16usize as usize))) as u32);
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        let mut i: u32 = 0_u32;
        'loop_: while (i < N) {
            assert!((unimplemented!() == unimplemented!()));
            let mut j: usize = 0_usize;
            'loop_: while (j < unimplemented!()) {
                assert!(((unimplemented!().read()) == (unimplemented!().read())));
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let a: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
            1_f64, 2.5_f64, 3_f64, 4.25_f64, 5_f64,
        ])));
        let mut N: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize)))
            as u32);
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        let mut i: usize = 0_usize;
        'loop_: while (i < unimplemented!()) {
            assert!(((unimplemented!().read()) == ((2 as f64) * (unimplemented!().read()))));
            i.prefix_inc();
        }
    }
    {
        let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 3_usize));
        assert!(((unimplemented!() == 1) && (unimplemented!() == 4)) && (unimplemented!() == 9));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
