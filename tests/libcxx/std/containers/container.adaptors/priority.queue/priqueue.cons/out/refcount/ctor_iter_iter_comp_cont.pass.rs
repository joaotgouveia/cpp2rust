extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 5, 2, 0, 6, 8, 1])));
    let mut n: i32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut v: Vec<i32> = {
        let mut __v = Vec::new();
        let mut __it = (a.as_pointer() as Ptr<i32>);
        while __it != (a.as_pointer() as Ptr<i32>).offset((n / 2) as isize) {
            __v.push((__it.iter_deref().read()));
            __it.iter_inc();
        }
        __v
    };
    let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == (n as ())));
    assert!(((unimplemented!().read()) == 8));
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
    assert!(({ test_0() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
