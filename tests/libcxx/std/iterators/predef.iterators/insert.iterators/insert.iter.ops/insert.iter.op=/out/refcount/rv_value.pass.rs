extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(c1: (), mut j: (), x1: (), x2: (), x3: (), c2: Ptr<()>) {
    let c1: Value<()> = Rc::new(RefCell::new(c1));
    let x1: Value<()> = Rc::new(RefCell::new(x1));
    let x2: Value<()> = Rc::new(RefCell::new(x2));
    let x3: Value<()> = Rc::new(RefCell::new(x3));
    let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn insert3at_1(c: Ptr<()>, i: (), x1: (), x2: (), x3: ()) {
    let i: Value<()> = Rc::new(RefCell::new(i));
    let x1: Value<()> = Rc::new(RefCell::new(x1));
    let x2: Value<()> = Rc::new(RefCell::new(x2));
    let x3: Value<()> = Rc::new(RefCell::new(x3));
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
            let _x1: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
            let _x2: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
            let _x3: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((5_()) as isize)
            });
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
            let _x2: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
            let _x3: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((5_()) as isize)
            });
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
            let _x1: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
            let _x2: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
            let _x3: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((5_()) as isize)
            });
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
            let _x2: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
            let _x3: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((5_()) as isize)
            });
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
            let _x1: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
            let _x2: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
            let _x3: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((5_()) as isize)
            });
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
            let _x2: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
            let _x3: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((5_()) as isize)
            });
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
            let _x1: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
            let _x2: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
            let _x3: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((5_()) as isize)
            });
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((3_()) as isize)
            });
            let _x2: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((4_()) as isize)
            });
            let _x3: () = std_unique_ptr_int__do_nothing_::new({
                (x.as_pointer() as Ptr<()>).offset((5_()) as isize)
            });
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
