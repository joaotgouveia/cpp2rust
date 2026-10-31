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
        let a1: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
            1.5_f64, -2.5_f64, 3.4_f64, -4.5_f64, -5_f64,
        ])));
        let mut a3: [f64; 5] = [1.5_f64, 2.5_f64, 3.4_f64, 4.5_f64, 5_f64];
        let mut N: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize)))
            as u32);
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        let mut i: usize = 0_usize;
        'loop_: while (i < unimplemented!()) {
            assert!(((unimplemented!().read()) == a3[(i) as usize]));
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
