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
    let i: Value<std_atomic_int_> = Rc::new(RefCell::new(<std_atomic_int_>::default()));
    let v: Value<std_atomic_int_> = Rc::new(RefCell::new(<std_atomic_int_>::default()));
    let exp: Value<i32> = Rc::new(RefCell::new(0));
    &({
        ((*i.borrow()) as std___atomic_base_int_).compare_exchange_weak_39(
            exp.as_pointer(),
            0,
            memory_order_acq_rel_0.with(|rc| *rc.borrow()),
        )
    });
    &({
        ((*i.borrow()) as std___atomic_base_int_).compare_exchange_weak_39(
            exp.as_pointer(),
            0,
            memory_order_release_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        ((*i.borrow()) as std___atomic_base_int_).compare_exchange_strong_43(
            exp.as_pointer(),
            0,
            memory_order_acq_rel_0.with(|rc| *rc.borrow()),
        )
    });
    ({
        ((*i.borrow()) as std___atomic_base_int_).compare_exchange_strong_43(
            exp.as_pointer(),
            0,
            memory_order_release_1.with(|rc| *rc.borrow()),
        )
    });
    &({
        ((*v.borrow()) as std___atomic_base_int_).compare_exchange_weak_40(
            exp.as_pointer(),
            0,
            memory_order_acq_rel_0.with(|rc| *rc.borrow()),
        )
    });
    &({
        ((*v.borrow()) as std___atomic_base_int_).compare_exchange_weak_40(
            exp.as_pointer(),
            0,
            memory_order_release_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        ((*v.borrow()) as std___atomic_base_int_).compare_exchange_strong_44(
            exp.as_pointer(),
            0,
            memory_order_acq_rel_0.with(|rc| *rc.borrow()),
        )
    });
    ({
        ((*v.borrow()) as std___atomic_base_int_).compare_exchange_strong_44(
            exp.as_pointer(),
            0,
            memory_order_release_1.with(|rc| *rc.borrow()),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
