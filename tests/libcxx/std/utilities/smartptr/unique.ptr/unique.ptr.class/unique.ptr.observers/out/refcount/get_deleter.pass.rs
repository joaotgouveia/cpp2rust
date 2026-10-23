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
pub struct test_macros_detail_is_same_const_Deleter_ref__const_Deleter_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_Deleter_ref__Deleter_ref_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Deleter {}
impl Deleter {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for Deleter {
    fn default() -> Self {
        { Deleter::new() }
    }
}
pub fn test_basic_2() {
    {
        let mut p: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut d: Deleter = Deleter::new();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Ptr<()> = p.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut d: Deleter = Deleter::new();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Ptr<()> = p.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_basic_3() {
    {
        let mut p: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut d: Deleter = Deleter::new();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Ptr<()> = p.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut d: Deleter = Deleter::new();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Ptr<()> = p.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_4() -> () {
    ({ test_basic_2() });
    ({ test_basic_3() });
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
pub trait DeleterImpl {
    fn operator_call(&self, _a0: ());
    fn test_3(&self) -> ();
    fn test_4(&self) -> ();
}
impl DeleterImpl for Ptr<Deleter> {
    fn operator_call(&self, mut _a0: ()) {}
    fn test_3(&self) -> () {
        return 5_();
    }
    fn test_4(&self) -> () {
        return 6_();
    }
}
pub fn __cpp2rust_init_globals() {}
