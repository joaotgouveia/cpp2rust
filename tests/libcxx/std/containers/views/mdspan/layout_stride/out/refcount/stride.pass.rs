extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_stride_0(strides: (), mut args: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        unimplemented!();
        r.postfix_inc();
    }
    const _: () = assert!(true, "");;
    let strides_out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(strides_out), std::array<typename E::index_type, E::rank()>>"
    );;
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_stride_1(strides: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        unimplemented!();
        r.postfix_inc();
    }
    const _: () = assert!(true, "");;
    let strides_out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(strides_out), std::array<typename E::index_type, E::rank()>>"
    );;
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_stride_2(strides: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        unimplemented!();
        r.postfix_inc();
    }
    const _: () = assert!(true, "");;
    let strides_out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(strides_out), std::array<typename E::index_type, E::rank()>>"
    );;
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_stride_3(strides: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        unimplemented!();
        r.postfix_inc();
    }
    const _: () = assert!(true, "");;
    let strides_out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(strides_out), std::array<typename E::index_type, E::rank()>>"
    );;
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_4() -> () {
    let mut D: () = dynamic_extent_5.with(|rc| *rc.borrow());
    ({ test_stride_0(unimplemented!(), 7_()) });
    ({ test_stride_1(unimplemented!()) });
    ({ test_stride_2(unimplemented!()) });
    ({ test_stride_3(unimplemented!(), 7_(), 9_(), 10_()) });
    return true;
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
    ({ test_4() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
