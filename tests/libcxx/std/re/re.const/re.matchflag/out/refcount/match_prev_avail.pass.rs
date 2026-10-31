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
    let str1: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"\na\0")));
    let mut str1_scnd: Ptr<i8> = (str1.as_pointer() as Ptr<i8>).offset((1) as isize);
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    let S: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b" cd").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let Start: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec(S.decay()).iter_add(1_isize),
    ));
    let End: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::Vec(S.decay().to_last())));
    assert!(unimplemented!());
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
