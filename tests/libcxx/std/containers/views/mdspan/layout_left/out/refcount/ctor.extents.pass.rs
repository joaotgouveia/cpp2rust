extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_construction_0(e: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        expected_size *= unimplemented!();
        r.postfix_inc();
    }
    unimplemented!();
}
pub fn test_construction_1(e: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        {
            let rhs_0 = (expected_size).wrapping_mul(unimplemented!());
            expected_size = rhs_0
        };
        r.postfix_inc();
    }
    unimplemented!();
}
pub fn test_construction_2(e: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        {
            let rhs_0 = (expected_size).wrapping_mul(unimplemented!());
            expected_size = rhs_0
        };
        r.postfix_inc();
    }
    unimplemented!();
}
pub fn test_construction_3(e: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        {
            let rhs_0 = (expected_size).wrapping_mul(unimplemented!());
            expected_size = rhs_0
        };
        r.postfix_inc();
    }
    unimplemented!();
}
pub fn test_construction_4(e: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        expected_size *= unimplemented!();
        r.postfix_inc();
    }
    unimplemented!();
}
pub fn test_5() -> () {
    let mut D: () = dynamic_extent_6.with(|rc| *rc.borrow());
    ({ test_construction_0(unimplemented!()) });
    ({ test_construction_1(unimplemented!()) });
    ({ test_construction_2(unimplemented!()) });
    ({ test_construction_3(unimplemented!()) });
    ({ test_construction_4(unimplemented!()) });
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
    ({ test_5() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
