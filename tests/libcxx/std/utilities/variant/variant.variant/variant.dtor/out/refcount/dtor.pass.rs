extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct NonTDtor {
    #[offset(0)]
    #[byte_size(8)]
    pub count: Ptr<()>,
}
impl NonTDtor {
    pub fn new(mut a: Ptr<()>, mut _a1: Ptr<()>) -> Self {
        Self { count: (a).clone() }
    }
}
const _: () = assert!(true, "!std::is_trivially_destructible<NonTDtor>::value");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct NonTDtor1 {
    #[offset(0)]
    #[byte_size(8)]
    pub count: Ptr<()>,
}
impl NonTDtor1 {
    pub fn new(mut _a0: Ptr<()>, mut b: Ptr<()>) -> Self {
        Self { count: (b).clone() }
    }
}
const _: () = assert!(true, "!std::is_trivially_destructible<NonTDtor1>::value");
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct TDtor {}
impl TDtor {
    pub fn copy_from(_a0: Ptr<TDtor>) -> Self {
        Self {}
    }
}
impl Clone for TDtor {
    fn clone(&self) -> Self {
        let __src: Value<TDtor> = Rc::new(RefCell::new(TDtor {}));
        TDtor::copy_from(__src.as_pointer())
    }
}
const _: () = assert!(true, "!std::is_trivially_copy_constructible<TDtor>::value");
const _: () = assert!(true, "std::is_trivially_destructible<TDtor>::value");
pub fn test_0() -> () {
    {
        const _: () = assert!(true, "std::is_trivially_destructible<V>::value");;
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    {
        const _: () = assert!(true, "!std::is_trivially_destructible<V>::value");;
        {
            let count0: Value<()> = Rc::new(RefCell::new(0_()));
            let count1: Value<()> = Rc::new(RefCell::new(0_()));
            {
                let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        {
            let mut count0: () = 0_();
            let mut count1: () = 0_();
            {
                let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            }
            unimplemented!();
            unimplemented!();
        }
        {
            let count0: Value<()> = Rc::new(RefCell::new(0_()));
            let count1: Value<()> = Rc::new(RefCell::new(0_()));
            {
                let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
    }
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait NonTDtorImpl {
    fn destructor(&self);
}
impl NonTDtorImpl for Ptr<NonTDtor> {
    fn destructor(&self) {
        (*self)
            .with(|__s| __s.count.clone())
            .with_mut(|__v| __v.prefix_inc());
    }
}
pub trait NonTDtor1Impl {
    fn destructor(&self);
}
impl NonTDtor1Impl for Ptr<NonTDtor1> {
    fn destructor(&self) {
        (*self)
            .with(|__s| __s.count.clone())
            .with_mut(|__v| __v.prefix_inc());
    }
}
pub trait TDtorImpl {
    fn copy_assign(&self, _a0: Ptr<TDtor>) -> Ptr<TDtor>;
}
impl TDtorImpl for Ptr<TDtor> {
    fn copy_assign(&self, _a0: Ptr<TDtor>) -> Ptr<TDtor> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
