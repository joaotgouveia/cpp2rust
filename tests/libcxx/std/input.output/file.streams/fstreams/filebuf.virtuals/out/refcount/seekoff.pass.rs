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
    {
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_i8).collect::<Box<[i8]>>()));
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        unimplemented!();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 11_()));
        assert!((unimplemented!() == (('l' as i8) as i32)));
        unimplemented!();
        assert!((unimplemented!() == (('a' as i8) as i32)));
        unimplemented!();
        assert!((unimplemented!() == (('z' as i8) as i32)));
        assert!(unimplemented!());
        assert!((unimplemented!() == (('l' as i8) as i32)));
    }
    unimplemented!();
    {
        let buf: Value<Box<[i32]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_i32).collect::<Box<[i32]>>()));
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        unimplemented!();
        const _: () = assert!(true, "");;
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 11_()));
        assert!((unimplemented!() == ((108 as i32) as ())));
        unimplemented!();
        assert!((unimplemented!() == ((97 as i32) as ())));
        unimplemented!();
        assert!((unimplemented!() == ((122 as i32) as ())));
        assert!(unimplemented!());
        assert!((unimplemented!() == ((108 as i32) as ())));
    }
    unimplemented!();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
