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
    assert!((unimplemented!() == 0_f64));
    assert!((unimplemented!() == 0_f64));
    assert!((unimplemented!() == (-10_i32 as f64)));
    assert!((unimplemented!() == 10_f64));
    {
        let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
        assert!((unimplemented!() == 10_f64));
        assert!(((*idx.borrow()) == 2_usize));
    }
    {
        let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
        assert!((unimplemented!() == 1e+60_f64));
        assert!(((*idx.borrow()) == 5_usize));
    }
    {
        let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
        assert!((unimplemented!() == ((f32::INFINITY) as f64)));
        assert!(((*idx.borrow()) == 3_usize));
    }
    {
        let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
        assert!(unimplemented!());
        assert!(((*idx.borrow()) == 3_usize));
    }
    assert!((unimplemented!() == 0_f64));
    assert!((unimplemented!() == 0_f64));
    assert!((unimplemented!() == -10.5_f64));
    assert!((unimplemented!() == 10_f64));
    {
        let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
        assert!((unimplemented!() == 10_f64));
        assert!(((*idx.borrow()) == 2_usize));
    }
    {
        let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
        assert!((unimplemented!() == 1e+60_f64));
        assert!(((*idx.borrow()) == 5_usize));
    }
    {
        let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
        assert!((unimplemented!() == ((f32::INFINITY) as f64)));
        assert!(((*idx.borrow()) == 3_usize));
    }
    {
        let idx: Value<usize> = Rc::new(RefCell::new(0_usize));
        assert!(unimplemented!());
        assert!(((*idx.borrow()) == 3_usize));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
