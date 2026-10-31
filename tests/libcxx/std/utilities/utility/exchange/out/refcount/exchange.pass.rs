extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_0() -> bool {
    let mut v: i32 = 12;
    if (12 != unimplemented!()) || (v != 23) {
        return false;
    }
    if (23 != unimplemented!()) || (v != 67) {
        return false;
    }
    if (67 != unimplemented!()) || (v != 0) {
        return false;
    }
    return true;
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexcept_true__true_ {}
impl TestNoexcept_true__true_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexcept_true__false_ {}
impl TestNoexcept_true__false_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexcept_false__true_ {}
impl TestNoexcept_false__true_ {}
pub fn test_noexcept_1() -> bool {
    {
        let mut x: i32 = 42;
        const _: () = assert!(true, "");;
    }
    {
        let mut x: TestNoexcept_true__true_ = <TestNoexcept_true__true_>::default();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut x: TestNoexcept_true__false_ = <TestNoexcept_true__false_>::default();
        const _: () = assert!(true, "");;
    }
    {
        let mut x: TestNoexcept_false__true_ = <TestNoexcept_false__true_>::default();
        const _: () = assert!(true, "");;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let mut v: i32 = 12;
        assert!((unimplemented!() == 12));
        assert!((v == 23));
        assert!((unimplemented!() == 23));
        assert!((v == 67));
        assert!(((unimplemented!()) == 67));
        assert!((v == 0));
    }
    {
        let mut b: bool = false;
        assert!(!(unimplemented!()));
        assert!(b);
    }
    {
        let mut s1: Vec<i8> = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"Hi Mom!").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        let mut s2: Vec<i8> = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"Yo Dad!").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        let s3: Value<Vec<i8>> = Rc::new(RefCell::new((s1).clone()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        s3.as_pointer().write((s2).clone());
        assert!(unimplemented!());
        assert!((((*s3.borrow()).len() - 1) == 0_usize));
        s3.as_pointer().write((s2).clone());
        assert!(unimplemented!());
        assert!((((*s3.borrow()).len() - 1) == 0_usize));
    }
    const _: () = assert!(true, "test_constexpr()");;
    const _: () = assert!(true, "test_noexcept()");;
    return 0;
}
pub trait TestNoexcept_false__true_Impl {
    fn copy_assign(&self, _a0: Ptr<TestNoexcept_false__true_>) -> Ptr<TestNoexcept_false__true_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<TestNoexcept_false__true_>) -> Ptr<TestNoexcept_false__true_> {
        unimplemented!()
    }
}
impl TestNoexcept_false__true_Impl for Ptr<TestNoexcept_false__true_> {}
pub trait TestNoexcept_true__false_Impl {
    fn copy_assign(&self, _a0: Ptr<TestNoexcept_true__false_>) -> Ptr<TestNoexcept_true__false_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<TestNoexcept_true__false_>) -> Ptr<TestNoexcept_true__false_> {
        unimplemented!()
    }
}
impl TestNoexcept_true__false_Impl for Ptr<TestNoexcept_true__false_> {}
pub trait TestNoexcept_true__true_Impl {
    fn copy_assign(&self, _a0: Ptr<TestNoexcept_true__true_>) -> Ptr<TestNoexcept_true__true_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<TestNoexcept_true__true_>) -> Ptr<TestNoexcept_true__true_> {
        unimplemented!()
    }
}
impl TestNoexcept_true__true_Impl for Ptr<TestNoexcept_true__true_> {}
pub fn __cpp2rust_init_globals() {}
