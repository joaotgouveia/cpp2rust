extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(s: Ptr<Vec<i8>>) {
    let sm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*({
        ((*sm.borrow())
            as (
                Value<RandomAccessIterator<i8>>,
                Value<RandomAccessIterator<i8>>,
            ))
    })
    .0
    .borrow_mut()) = unimplemented!();
    (*({
        ((*sm.borrow())
            as (
                Value<RandomAccessIterator<i8>>,
                Value<RandomAccessIterator<i8>>,
            ))
    })
    .1
    .borrow_mut()) = unimplemented!();
    (*sm.borrow_mut()).matched = true;
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!(unimplemented!());
}
pub fn test_1(s: Ptr<Vec<i32>>) {
    let sm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*({
        ((*sm.borrow())
            as (
                Value<RandomAccessIterator<i32>>,
                Value<RandomAccessIterator<i32>>,
            ))
    })
    .0
    .borrow_mut()) = unimplemented!();
    (*({
        ((*sm.borrow())
            as (
                Value<RandomAccessIterator<i32>>,
                Value<RandomAccessIterator<i32>>,
            ))
    })
    .1
    .borrow_mut()) = unimplemented!();
    (*sm.borrow_mut()).matched = true;
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!(unimplemented!());
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
    ({
        let _s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"123").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        test_0(_s.as_pointer())
    });
    ({
        let _s: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes =
                Ptr::<i32>::from_string_literal(&[49 as i32, 50 as i32, 51 as i32, 0 as i32])
                    .to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        test_1(_s.as_pointer())
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
