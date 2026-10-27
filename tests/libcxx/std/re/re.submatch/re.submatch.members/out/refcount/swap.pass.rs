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
    {
        let s1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('1' as ()),
            ('2' as ()),
            ('3' as ()),
            0_(),
        ])));
        let sm1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*sm1.borrow_mut()).first = (s1.as_pointer() as Ptr<()>);
        (*sm1.borrow_mut()).second = (s1.as_pointer() as Ptr<()>).offset((3_()) as isize);
        (*sm1.borrow_mut()).matched = true;
        let sm2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('c' as ()),
            ('a' as ()),
            ('t' as ()),
            0_(),
        ])));
        (*sm2.borrow_mut()).first = (s2.as_pointer() as Ptr<()>);
        (*sm2.borrow_mut()).second = (s2.as_pointer() as Ptr<()>).offset((3_()) as isize);
        (*sm2.borrow_mut()).matched = false;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let s1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            (49 as ()),
            (50 as ()),
            (51 as ()),
            0_(),
        ])));
        let sm1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*sm1.borrow_mut()).first = (s1.as_pointer() as Ptr<()>);
        (*sm1.borrow_mut()).second = (s1.as_pointer() as Ptr<()>).offset((3_()) as isize);
        (*sm1.borrow_mut()).matched = true;
        let sm2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            (99 as ()),
            (97 as ()),
            (116 as ()),
            0_(),
        ])));
        (*sm2.borrow_mut()).first = (s2.as_pointer() as Ptr<()>);
        (*sm2.borrow_mut()).second = (s2.as_pointer() as Ptr<()>).offset((3_()) as isize);
        (*sm2.borrow_mut()).matched = false;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    0
}
pub fn __cpp2rust_init_globals() {}
