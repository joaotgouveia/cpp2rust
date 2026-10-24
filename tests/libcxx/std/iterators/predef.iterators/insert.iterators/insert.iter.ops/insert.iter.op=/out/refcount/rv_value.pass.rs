extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(c1: (), mut j: (), mut x1: (), mut x2: (), mut x3: (), c2: Ptr<()>) {
    let c1: Value<()> = Rc::new(RefCell::new(c1));
    let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn insert3at_1(c: Ptr<()>, i: (), mut x1: (), mut x2: (), mut x3: ()) {
    let i: Value<()> = Rc::new(RefCell::new(i));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct do_nothing {}
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
    {
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            0_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        ({
            let _c: Ptr<()> = c2.as_pointer();
            let _i: () = unimplemented!();
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            test_0(unimplemented!(), 0_(), _x1, _x2, _x3, c2.as_pointer())
        });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        ({
            let _c: Ptr<()> = c2.as_pointer();
            let _i: () = unimplemented!();
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            test_0(unimplemented!(), 1_(), _x1, _x2, _x3, c2.as_pointer())
        });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        ({
            let _c: Ptr<()> = c2.as_pointer();
            let _i: () = unimplemented!();
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            test_0(unimplemented!(), 2_(), _x1, _x2, _x3, c2.as_pointer())
        });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        ({
            let _c: Ptr<()> = c2.as_pointer();
            let _i: () = unimplemented!();
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: () = unimplemented!();
            let _x2: () = unimplemented!();
            let _x3: () = unimplemented!();
            test_0(unimplemented!(), 3_(), _x1, _x2, _x3, c2.as_pointer())
        });
    }
    return 0_();
}
pub trait do_nothingImpl {
    fn operator_call(&self, _a0: ());
}
impl do_nothingImpl for Ptr<do_nothing> {
    fn operator_call(&self, mut _a0: ()) {}
}
pub fn __cpp2rust_init_globals() {}
