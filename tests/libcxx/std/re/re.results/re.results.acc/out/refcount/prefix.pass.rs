extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
    assert!(unimplemented!());
    assert!(
        ({
            (*((unimplemented!()).clone() as (Value<*const i8>, Value<*const i8>))
                .0
                .borrow())
            .clone()
        } == { (s.as_pointer() as Ptr::<i8>) })
    );
    assert!(
        ({
            (*((unimplemented!()).clone() as (Value<*const i8>, Value<*const i8>))
                .1
                .borrow())
            .clone()
        } == { (s.as_pointer() as Ptr::<i8>).offset((2) as isize) })
    );
    assert!((({ (unimplemented!()).clone().matched } as i32) == (true as i32)));
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
    ({ test_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
