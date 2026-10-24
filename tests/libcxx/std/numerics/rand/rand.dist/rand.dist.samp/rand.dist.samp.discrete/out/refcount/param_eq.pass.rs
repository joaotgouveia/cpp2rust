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
        let p0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([30_(), 10_()])));
        let p1: Value<()> = Rc::new(RefCell::new(std_discrete_distribution___param_type::new_1(
            { (p0.as_pointer() as Ptr<()>) },
            { (p0.as_pointer() as Ptr<()>).offset((2_()) as isize) },
        )));
        let p2: Value<()> = Rc::new(RefCell::new(std_discrete_distribution___param_type::new_1(
            { (p0.as_pointer() as Ptr<()>) },
            { (p0.as_pointer() as Ptr<()>).offset((2_()) as isize) },
        )));
        unimplemented!();
    }
    {
        let p0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([30_(), 10_()])));
        let p1: Value<()> = Rc::new(RefCell::new(std_discrete_distribution___param_type::new_1(
            { (p0.as_pointer() as Ptr<()>) },
            { (p0.as_pointer() as Ptr<()>).offset((2_()) as isize) },
        )));
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
