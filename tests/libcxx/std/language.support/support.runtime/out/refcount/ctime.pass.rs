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
    let mut c: () = 0_();
    let mut s: () = 0_();
    let t: Value<()> = Rc::new(RefCell::new(0_()));
    let tm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(c));
    (&(s));
    (&(*t.borrow_mut()));
    (&(*tm.borrow_mut()));
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::clock()), std::clock_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::difftime(t,t)), double>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::mktime(&tm)), std::time_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::time(&t)), std::time_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::asctime(&tm)), char*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::ctime(&t)), char*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::gmtime(&t)), std::tm*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::localtime(&t)), std::tm*>::value)"
    );;
    let mut c1: Ptr<()> = Ptr::<()>::null();
    let mut c2: Ptr<()> = Ptr::<()>::null();
    (&(c1));
    (&(c2));
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::strftime(c1,s,c2,&tm)), std::size_t>::value)"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
