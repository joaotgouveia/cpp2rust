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
    let a1: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([1_usize, 2_usize, 3_usize])));
    let a2: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([4_usize, 5_usize, 6_usize])));
    let size: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let stride: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let gs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == 7_usize));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == 3_usize));
    assert!(((unimplemented!().read()) == 1_u64));
    assert!(((unimplemented!().read()) == 2_u64));
    assert!(((unimplemented!().read()) == 3_u64));
    unimplemented!();
    assert!((unimplemented!() == 3_usize));
    assert!(((unimplemented!().read()) == 4_u64));
    assert!(((unimplemented!().read()) == 5_u64));
    assert!(((unimplemented!().read()) == 6_u64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
