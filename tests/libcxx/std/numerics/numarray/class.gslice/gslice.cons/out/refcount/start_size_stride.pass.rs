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
    let size: Value<std_valarray_unsigned_long_> =
        Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1(
            { (a1.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>() },
            {
                (::std::mem::size_of::<[usize; 3]>() as usize)
                    .wrapping_div((::std::mem::size_of::<usize>() as usize))
            },
        )));
    let stride: Value<std_valarray_unsigned_long_> =
        Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1(
            { (a2.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>() },
            {
                (::std::mem::size_of::<[usize; 3]>() as usize)
                    .wrapping_div((::std::mem::size_of::<usize>() as usize))
            },
        )));
    let gs: Value<std_gslice> = Rc::new(RefCell::new(std_gslice::new_1(
        { 7_usize },
        { size.as_pointer() },
        { stride.as_pointer() },
    )));
    assert!(({ (({ (*gs.borrow()).start() }) == 7_usize) }));
    let r: Value<std_valarray_unsigned_long_> = Rc::new(RefCell::new(({ (*gs.borrow()).size() })));
    assert!(({ (({ (*r.borrow()).size() }) == 3_usize) }));
    assert!(({ ({ (*r.borrow())[0_usize] } == 1_u64) }));
    assert!(({ ({ (*r.borrow())[1_usize] } == 2_u64) }));
    assert!(({ ({ (*r.borrow())[2_usize] } == 3_u64) }));
    (*r.borrow_mut()) = ({ (*gs.borrow()).stride() });
    assert!(({ (({ (*r.borrow()).size() }) == 3_usize) }));
    assert!(({ ({ (*r.borrow())[0_usize] } == 4_u64) }));
    assert!(({ ({ (*r.borrow())[1_usize] } == 5_u64) }));
    assert!(({ ({ (*r.borrow())[2_usize] } == 6_u64) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
