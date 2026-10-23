extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_unget_0(mut fb: Ptr<()>, mut expected: ()) {
    let mut c: () = unimplemented!();
    if (c != (-1_())) {
        unimplemented!();
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
    let mut stream_buffer: () = unimplemented!();
    let fs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut fb: Ptr<()> = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut test_buffer: () = unimplemented!();
    unimplemented!().write(('\0' as ()));
    {
        unimplemented!();
        unimplemented!();
        ({ check_unget_0((fb).clone(), (' ' as ())) });
    }
    {
        unimplemented!();
        unimplemented!();
        ({ check_unget_0((fb).clone(), ('o' as ())) });
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ check_unget_0((fb).clone(), ('a' as ())) });
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ check_unget_0((fb).clone(), ('a' as ())) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ check_unget_0((fb).clone(), (' ' as ())) });
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ check_unget_0((fb).clone(), ('r' as ())) });
    }
    {
        let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let ifs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut filebuf: Ptr<()> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
