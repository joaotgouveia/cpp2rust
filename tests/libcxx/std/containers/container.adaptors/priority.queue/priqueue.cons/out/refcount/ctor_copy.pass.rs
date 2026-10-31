extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_0(mut n: i32) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        {
            let a0_clone = i.clone();
            c.push(a0_clone)
        };
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub fn test_1() -> bool {
    let mut v: Vec<i32> = ({ make_0(5) });
    let qo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == 5_()));
    assert!(((unimplemented!().read()) == 0));
    return true;
}
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
    assert!(({ test_1() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
