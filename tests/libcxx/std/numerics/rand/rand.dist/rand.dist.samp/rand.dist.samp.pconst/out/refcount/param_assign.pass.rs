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
        let b: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([10_(), 14_(), 16_(), 17_()])));
        let p: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([25_(), 62.5_f64, 12.5_f64])));
        let mut Np: () = (::std::mem::size_of::<[(); 3]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let p0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
