extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    {
        let a: Value<Box<[u32]>> =
            Rc::new(RefCell::new(Box::new([5_u32, 4_u32, 3_u32, 2_u32, 1_u32])));
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 5_usize));
        let b: Value<Box<[u32]>> =
            Rc::new(RefCell::new(Box::new([0_u32, 0_u32, 0_u32, 0_u32, 0_u32])));
        unimplemented!();
        assert!(((*b.borrow())[(0) as usize] == 5_u32));
        assert!(((*b.borrow())[(1) as usize] == 4_u32));
        assert!(((*b.borrow())[(2) as usize] == 3_u32));
        assert!(((*b.borrow())[(3) as usize] == 2_u32));
        assert!(((*b.borrow())[(4) as usize] == 1_u32));
    }
    {
        let a: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
            1311673392922361856_u64,
            78187493376_u64,
            18446744073709551615_u64,
            6442450944_u64,
        ])));
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 4_usize));
        let b: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([0_u32, 0_u32, 0_u32, 0_u32])));
        unimplemented!();
        assert!(((*b.borrow())[(0) as usize] == 1450704896_u32));
        assert!(((*b.borrow())[(1) as usize] == 878082048_u32));
        assert!(((*b.borrow())[(2) as usize] == 4294967295_u32));
        assert!(((*b.borrow())[(3) as usize] == 2147483648_u32));
    }
    {
        let a: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 3_usize));
        let b: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([0_u32, 0_u32, 0_u32])));
        unimplemented!();
        assert!(((*b.borrow())[(0) as usize] == 1_u32));
        assert!(((*b.borrow())[(1) as usize] == 2_u32));
        assert!(((*b.borrow())[(2) as usize] == 3_u32));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
