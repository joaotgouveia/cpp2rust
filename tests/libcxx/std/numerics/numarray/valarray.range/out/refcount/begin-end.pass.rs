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
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
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
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cv: Ptr<()> = v.as_pointer();
        const _: () = assert!(
            true,
            "std::is_base_of<std::random_access_iterator_tag, std::iterator_traits<It>::iterator_category>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_base_of<std::random_access_iterator_tag, std::iterator_traits<CIt>::iterator_category>::value"
        );;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!().write(10_());
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut sum: () = 0_();
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let i: Ptr<()> = (i).clone();
            sum += { (i.read()) };
        }
        unimplemented!();
    }
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
        let cv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut sum: () = 0_();
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let i: Ptr<()> = (i).clone();
            sum += { (i.read()) };
        }
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
