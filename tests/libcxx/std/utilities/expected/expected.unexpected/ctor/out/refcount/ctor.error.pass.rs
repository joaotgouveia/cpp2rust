extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(true, "std::constructible_from<std::unexpected<int>, int>");
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CstrFromUnexpected {}
impl CstrFromUnexpected {}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<CstrFromUnexpected>, std::unexpected<CstrFromUnexpected>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CstrFromInplace {}
impl CstrFromInplace {}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<CstrFromInplace>, std::in_place_t>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Foo {}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<Foo>, int>"
);
const _: () = assert!(true, "std::convertible_to<int, int>");
const _: () = assert!(!(false), "!std::convertible_to<int, std::unexpected<int>>");
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct Error {
    #[offset(0)]
    pub i: (),
}
impl Error {
    pub fn new_1(mut ii: ()) -> Self {
        Self { i: ii }
    }
    pub fn copy_from(other: Ptr<Error>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
    pub fn move_from(other: Ptr<Error>) -> Self {
        let __this: Error = Self {
            i: other.with(|__s| __s.i),
        };
        field!(other, i).write(0_());
        __this
    }
    pub fn new_2(_a0: ()) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        let __this: Error = Self {
            i: unimplemented!(),
        };
        unimplemented!();
        __this
    }
}
impl Clone for Error {
    fn clone(&self) -> Self {
        let __src: Value<Error> = Rc::new(RefCell::new(Error { i: self.i.clone() }));
        Error::copy_from(__src.as_pointer())
    }
}
pub fn test_0() -> () {
    {
        let mut e: Error = Error::new_1({ 5_() });
        let unex: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let mut e: Error = Error::new_1({ 5_() });
        let unex: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let mut e: Error = Error::new_1({ 5_() });
        let unex: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    {
        let ue: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Bar_1 {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub j: (),
}
impl Bar_1 {
    pub fn new(mut ii: (), mut jj: ()) -> Self {
        Self { i: ii, j: jj }
    }
}
pub fn testException_2() {}
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
    ({ testException_2() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
