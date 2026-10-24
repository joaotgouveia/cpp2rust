extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_filesystem_path_ref__const_std_filesystem_path_ref_
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub fn test_constructors_2() {
    {
        let mut what_arg: () = unimplemented!();
        let what_contains: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let CheckWhat: Value<FnPtr<fn(Ptr<()>)>> = Rc::new(RefCell::new(lambda!(
            {
                let what_contains: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            },
            |e: Ptr<()>| {
                let mut s: () = unimplemented!();
                unimplemented!();
            }
        )));
        let ec: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            const _: () = assert!(true, "");;
            let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ (*CheckWhat.borrow()).call(e.as_pointer()) });
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "");;
            let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ (*CheckWhat.borrow()).call(e.as_pointer()) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "");;
            let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ (*CheckWhat.borrow()).call(e.as_pointer()) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut what_arg: () = unimplemented!();
        let what_contains: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let CheckWhat: Value<FnPtr<fn(Ptr<()>)>> = Rc::new(RefCell::new(lambda!(
            {
                let what_contains: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            },
            |e: Ptr<()>| {
                let mut s: () = unimplemented!();
                unimplemented!();
            }
        )));
        let ec: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            const _: () = assert!(true, "");;
            let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ (*CheckWhat.borrow()).call(e.as_pointer()) });
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "");;
            let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ (*CheckWhat.borrow()).call(e.as_pointer()) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "");;
            let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ (*CheckWhat.borrow()).call(e.as_pointer()) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_signatures_3() {
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ec: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
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
    const _: () = assert!(
        true,
        "std::is_base_of<std::system_error, fs::filesystem_error>::value"
    );;
    ({ test_constructors_2() });
    ({ test_signatures_3() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
