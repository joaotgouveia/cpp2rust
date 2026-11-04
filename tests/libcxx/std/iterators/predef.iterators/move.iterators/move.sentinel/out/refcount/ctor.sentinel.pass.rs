extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    {
        const _: () = assert!(true, "!std::is_convertible_v<int, std::move_sentinel<int>>");;
        let m: Value<std_move_sentinel_int_> =
            Rc::new(RefCell::new(std_move_sentinel_int_::new_1({ 42 })));
        assert!(({ (({ (*m.borrow()).base() }) == 42) }));
    }
    {
        const _: () = assert!(
            true,
            "!std::is_convertible_v<int*, std::move_sentinel<int*>>"
        );;
        let i: Value<i32> = Rc::new(RefCell::new(42));
        let m: Value<std_move_sentinel_int_ptr_> =
            Rc::new(RefCell::new(std_move_sentinel_int_ptr_::new_1({
                (i.as_pointer())
            })));
        assert!(({ (({ (*m.borrow()).base() }) == (i.as_pointer())) }));
    }
    {
        const _: () = assert!(true, "!std::is_convertible_v<S, std::move_sentinel<S>>");;
        let m: Value<std_move_sentinel_S_> = Rc::new(RefCell::new(std_move_sentinel_S_::new_1({
            S_1::new_1({ 42 })
        })));
        assert!(({ ({ (*m.borrow()).base() }).i } == 42));
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct S_1 {
    #[offset(0)]
    pub i: i32,
}
impl S_1 {
    pub fn new_1(mut j: i32) -> Self {
        Self { i: j }
    }
}
impl CppMove for S_1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S_1 {
    fn default() -> Self {
        { S_1 { i: 3 } }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
