extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_close_0(mut v: (), mut comp: ()) -> () {
    return (v <= (comp + unimplemented!())) && (v >= (comp - unimplemented!()));
}
pub fn is_close_1(mut v: (), mut comp: ()) -> () {
    return (v <= (comp + unimplemented!())) && (v >= (comp - unimplemented!()));
}
pub fn is_close_2(mut v: (), mut comp: ()) -> () {
    return (v <= (comp + unimplemented!())) && (v >= (comp - unimplemented!()));
}
pub fn test_3() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*c3.borrow_mut()) = (*c.borrow()).clone();
    let ic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*c3.borrow_mut()) = (*c.borrow()).clone();
    let fc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_4() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*c3.borrow_mut()) = (*c.borrow()).clone();
    let ic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*c3.borrow_mut()) = (*c.borrow()).clone();
    let fc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_5() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*c3.borrow_mut()) = (*c.borrow()).clone();
    let ic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*c3.borrow_mut()) = (*c.borrow()).clone();
    let fc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    const _: () = assert!(true, "test<float>()");;
    const _: () = assert!(true, "test<double>()");;
    const _: () = assert!(true, "test<long double>()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
