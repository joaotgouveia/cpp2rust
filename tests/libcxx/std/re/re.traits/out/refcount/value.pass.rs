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
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut c: i8 = 0_i8;
        'loop_: while ((c as i32) < (('0' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            c.prefix_inc();
        }
        let mut c: i8 = ('0' as i8);
        'loop_: while ((c as i32) < (('8' as i8) as i32)) {
            assert!((unimplemented!() == ((c as i32) - (('0' as i8) as i32))));
            assert!((unimplemented!() == ((c as i32) - (('0' as i8) as i32))));
            assert!((unimplemented!() == ((c as i32) - (('0' as i8) as i32))));
            c.prefix_inc();
        }
        let mut c: i8 = ('8' as i8);
        'loop_: while ((c as i32) < ((':' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == ((c as i32) - (('0' as i8) as i32))));
            assert!((unimplemented!() == ((c as i32) - (('0' as i8) as i32))));
            c.prefix_inc();
        }
        let mut c: i8 = (':' as i8);
        'loop_: while ((c as i32) < (('A' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            c.prefix_inc();
        }
        let mut c: i8 = ('A' as i8);
        'loop_: while ((c as i32) < (('G' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == (((c as i32) - (('A' as i8) as i32)) + 10)));
            c.prefix_inc();
        }
        let mut c: i8 = ('G' as i8);
        'loop_: while ((c as i32) < (('a' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            c.prefix_inc();
        }
        let mut c: i8 = ('a' as i8);
        'loop_: while ((c as i32) < (('g' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == (((c as i32) - (('a' as i8) as i32)) + 10)));
            c.prefix_inc();
        }
        let mut c: i32 = (('g' as i8) as i32);
        'loop_: while (c < 256) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            c.prefix_inc();
        }
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut c: i32 = 0_i32;
        'loop_: while (c < (('0' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            c.prefix_inc();
        }
        let mut c: i32 = (('0' as i8) as i32);
        'loop_: while (c < (('8' as i8) as i32)) {
            assert!((unimplemented!() == (c - (('0' as i8) as i32))));
            assert!((unimplemented!() == (c - (('0' as i8) as i32))));
            assert!((unimplemented!() == (c - (('0' as i8) as i32))));
            c.prefix_inc();
        }
        let mut c: i32 = (('8' as i8) as i32);
        'loop_: while (c < ((':' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == (c - (('0' as i8) as i32))));
            assert!((unimplemented!() == (c - (('0' as i8) as i32))));
            c.prefix_inc();
        }
        let mut c: i32 = ((':' as i8) as i32);
        'loop_: while (c < (('A' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            c.prefix_inc();
        }
        let mut c: i32 = (('A' as i8) as i32);
        'loop_: while (c < (('G' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == ((c - (('A' as i8) as i32)) + 10)));
            c.prefix_inc();
        }
        let mut c: i32 = (('G' as i8) as i32);
        'loop_: while (c < (('a' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            c.prefix_inc();
        }
        let mut c: i32 = (('a' as i8) as i32);
        'loop_: while (c < (('g' as i8) as i32)) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == ((c - (('a' as i8) as i32)) + 10)));
            c.prefix_inc();
        }
        let mut c: i32 = (('g' as i8) as i32);
        'loop_: while (c < 65535) {
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            assert!((unimplemented!() == -1_i32));
            c.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
