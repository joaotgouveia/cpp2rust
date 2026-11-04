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
impl CppMove for CstrFromInplace {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<CstrFromInplace>, std::in_place_t>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Foo {}
impl CppMove for Foo {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::unexpected<Foo>, int>"
);
const _: () = assert!(true, "std::convertible_to<int, int>");
const _: () = assert!(!(false), "!std::convertible_to<int, std::unexpected<int>>");
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new_1, value)]
#[cpp_from(Vec<Error  >  , new_2, value)]
#[byte_size(4)]
pub struct Error {
    #[offset(0)]
    pub i: i32,
}
impl Error {
    pub fn new_1(mut ii: i32) -> Self {
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
        field!(other, i).write(0);
        __this
    }
    pub fn new_2(_a0: Vec<Error>) -> Self {
        let _a0: Value<Vec<Error>> = Rc::new(RefCell::new(_a0));
        let __this: Error = Self { i: 0_i32 };
        assert!(false);
        __this
    }
}
impl Clone for Error {
    fn clone(&self) -> Self {
        let __src: Value<Error> = Rc::new(RefCell::new(Error { i: self.i.clone() }));
        Error::copy_from(__src.as_pointer())
    }
}
impl CppMove for Error {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn test_0() -> bool {
    {
        let e: Value<Error> = Rc::new(RefCell::new(Error::new_1({ 5 })));
        let unex: Value<std_unexpected_Error_> =
            Rc::new(RefCell::new(std_unexpected_Error_::new_1({
                e.as_pointer()
            })));
        assert!(({ (({ (*unex.borrow()).error_9() }).with(|__s| __s.i) == 5) }));
        assert!(({ ({ (*e.borrow()).i } == 5) }));
    }
    {
        let e: Value<Error> = Rc::new(RefCell::new(Error::new_1({ 5 })));
        let unex: Value<std_unexpected_Error_> =
            Rc::new(RefCell::new(std_unexpected_Error_::new_2({
                e.as_pointer()
            })));
        assert!(({ (({ (*unex.borrow()).error_9() }).with(|__s| __s.i) == 5) }));
        assert!(({ ({ (*e.borrow()).i } == 0) }));
    }
    {
        let e: Value<Error> = Rc::new(RefCell::new(Error::new_1({ 5 })));
        let unex: Value<std_unexpected_Error_> =
            Rc::new(RefCell::new(std_unexpected_Error_::new_1({
                e.as_pointer()
            })));
    }
    {
        let ue: Value<std_unexpected_Bar_> = Rc::new(RefCell::new({
            let __tmp_0: Value<Bar_1> = Rc::new(RefCell::new(Bar_1::new({ 5 }, { 6 })));
            std_unexpected_Bar_::new({ __tmp_0.as_pointer() })
        }));
        assert!(({ (({ (*ue.borrow()).error_9() }).with(|__s| __s.i) == 5) }));
        assert!(({ (({ (*ue.borrow()).error_9() }).with(|__s| __s.j) == 6) }));
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Bar_1 {
    #[offset(0)]
    pub i: i32,
    #[offset(4)]
    pub j: i32,
}
impl Bar_1 {
    pub fn new(mut ii: i32, mut jj: i32) -> Self {
        Self { i: ii, j: jj }
    }
}
impl CppMove for Bar_1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    ({ testException_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
