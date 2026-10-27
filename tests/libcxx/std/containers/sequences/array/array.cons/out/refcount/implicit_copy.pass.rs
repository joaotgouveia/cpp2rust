extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoDefault {}
impl NoDefault {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(1)]
pub struct NonTrivialCopy {}
impl NonTrivialCopy {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<NonTrivialCopy>) -> Self {
        Self {}
    }
}
impl Clone for NonTrivialCopy {
    fn clone(&self) -> Self {
        let __src: Value<NonTrivialCopy> = Rc::new(RefCell::new(NonTrivialCopy {}));
        NonTrivialCopy::copy_from(__src.as_pointer())
    }
}
impl Default for NonTrivialCopy {
    fn default() -> Self {
        { NonTrivialCopy::new() }
    }
}
pub fn tests_0() -> () {
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        (*copy.borrow_mut()) = (*array.borrow()).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "std::is_copy_assignable<Array>::value");;
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        &(*copy.borrow_mut());
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "");;
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        (*copy.borrow_mut()) = (*array.borrow()).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "std::is_copy_assignable<Array>::value");;
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        &(*copy.borrow_mut());
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "");;
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        (*copy.borrow_mut()) = (*array.borrow()).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "std::is_copy_assignable<Array>::value");;
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        &(*copy.borrow_mut());
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
        const _: () = assert!(true, "");;
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        (*copy.borrow_mut()) = (*array.borrow()).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        (*copy.borrow_mut()) = (*array.borrow()).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new((*array.borrow()).clone()));
        (*copy.borrow_mut()) = (*array.borrow()).clone();
        const _: () = assert!(true, "std::is_copy_constructible<Array>::value");;
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
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub trait NonTrivialCopyImpl {
    fn copy_assign(&self, _a0: Ptr<NonTrivialCopy>) -> Ptr<NonTrivialCopy>;
}
impl NonTrivialCopyImpl for Ptr<NonTrivialCopy> {
    fn copy_assign(&self, _a0: Ptr<NonTrivialCopy>) -> Ptr<NonTrivialCopy> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
