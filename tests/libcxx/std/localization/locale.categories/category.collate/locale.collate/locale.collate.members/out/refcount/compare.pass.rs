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
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let ia: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"1234\0")));
        let mut sa: u32 = (((::std::mem::size_of::<[i8; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i8>() as usize)))
            as u32);
        let ib: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"123\0")));
        let f: Ptr<()> = unimplemented!();
        assert!((unimplemented!() == 1));
        assert!((unimplemented!() == -1_i32));
        assert!((unimplemented!() == 1));
        assert!((unimplemented!() == -1_i32));
        assert!((unimplemented!() == -1_i32));
        assert!((unimplemented!() == 1));
        assert!((unimplemented!() == 0));
    }
    {
        let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            49 as i32, 50 as i32, 51 as i32, 52 as i32, 0 as i32,
        ])));
        let mut sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let ib: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            49 as i32, 50 as i32, 51 as i32, 0 as i32,
        ])));
        let f: Ptr<()> = unimplemented!();
        assert!((unimplemented!() == 1));
        assert!((unimplemented!() == -1_i32));
        assert!((unimplemented!() == 1));
        assert!((unimplemented!() == -1_i32));
        assert!((unimplemented!() == -1_i32));
        assert!((unimplemented!() == 1));
        assert!((unimplemented!() == 0));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
