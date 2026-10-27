extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_containers_0() {
    {
        let mut lhs: () = unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ret: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut lhs: () = unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ret: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut lhs: () = unimplemented!();
        unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let mut lhs: () = unimplemented!();
        unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!().write(-1_());
        unimplemented!();
    }
    {
        let mut lhs: () = unimplemented!();
        unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut lhs: () = unimplemented!();
        unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!().write(-1_());
        unimplemented!();
    }
}
pub fn test_containers_1() {
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: () = unimplemented!();
        let mut ret: () = unimplemented!();
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: () = unimplemented!();
        let mut ret: () = unimplemented!();
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut rhs: () = unimplemented!();
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut rhs: () = unimplemented!();
        unimplemented!().write(-1_());
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut rhs: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut rhs: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!().write(-1_());
        unimplemented!();
    }
}
pub fn test_containers_2() {
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ret: () = unimplemented!();
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ret: () = unimplemented!();
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!().write(-1_());
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!().write(-1_());
        unimplemented!();
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_containers_0() });
    ({ test_containers_1() });
    ({ test_containers_2() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
