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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let exp: Value<()> = Rc::new(RefCell::new(0_()));
    &(unimplemented!());
    &(unimplemented!());
    unimplemented!();
    unimplemented!();
    &({
        ((*v.borrow()) as std___atomic_base_int__false_).compare_exchange_weak_15(
            exp.as_pointer(),
            0_(),
            memory_order_acq_rel_0.with(|rc| *rc.borrow()),
        )
    });
    &({
        ((*v.borrow()) as std___atomic_base_int__false_).compare_exchange_weak_15(
            exp.as_pointer(),
            0_(),
            memory_order_release_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        ((*v.borrow()) as std___atomic_base_int__false_).compare_exchange_strong_17(
            exp.as_pointer(),
            0_(),
            memory_order_acq_rel_0.with(|rc| *rc.borrow()),
        )
    });
    ({
        ((*v.borrow()) as std___atomic_base_int__false_).compare_exchange_strong_17(
            exp.as_pointer(),
            0_(),
            memory_order_release_1.with(|rc| *rc.borrow()),
        )
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
