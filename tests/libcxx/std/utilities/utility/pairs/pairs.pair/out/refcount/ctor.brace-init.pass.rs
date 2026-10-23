extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct ExplicitT {
    #[offset(0)]
    pub value: (),
}
impl ExplicitT {
    pub fn new(mut x: ()) -> Self {
        Self { value: x }
    }
    pub fn copy_from(o: Ptr<ExplicitT>) -> Self {
        Self {
            value: o.with(|__s| __s.value),
        }
    }
}
impl Clone for ExplicitT {
    fn clone(&self) -> Self {
        let __src: Value<ExplicitT> = Rc::new(RefCell::new(ExplicitT {
            value: self.value.clone(),
        }));
        ExplicitT::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct ImplicitT {
    #[offset(0)]
    pub value: (),
}
impl ImplicitT {
    pub fn new(mut x: ()) -> Self {
        Self { value: x }
    }
    pub fn copy_from(o: Ptr<ImplicitT>) -> Self {
        Self {
            value: o.with(|__s| __s.value),
        }
    }
}
impl Clone for ImplicitT {
    fn clone(&self) -> Self {
        let __src: Value<ImplicitT> = Rc::new(RefCell::new(ImplicitT {
            value: self.value.clone(),
        }));
        ImplicitT::copy_from(__src.as_pointer())
    }
}
pub fn can_construct_with_brace_init_0(mut _a0: ()) -> () {
    return true;
}
pub fn can_construct_with_brace_init_1(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_construct_with_ctad_brace_init_2(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_construct_with_ctad_brace_init_3(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BraceInit {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoBraceInit {}
impl NoBraceInit {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitBraceInit {}
pub fn explicit_vs_implicit_brace_init_4(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return 1_();
}
pub fn explicit_vs_implicit_brace_init_5(_a0: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return 2_();
}
pub fn test_6() -> () {
    {
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*p.borrow_mut());
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*p.borrow_mut());
    }
    {
        const _: () = assert!(true, "can_construct_with_brace_init<BraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_brace_init<NoBraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_ctad_brace_init<BraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_ctad_brace_init<int>(0)");;
    }
    {
        unimplemented!();
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
    ({ test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait ExplicitTImpl {
    fn copy_assign(&self, _a0: Ptr<ExplicitT>) -> Ptr<ExplicitT>;
}
impl ExplicitTImpl for Ptr<ExplicitT> {
    fn copy_assign(&self, _a0: Ptr<ExplicitT>) -> Ptr<ExplicitT> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        return (*self).clone();
    }
}
pub trait ImplicitTImpl {
    fn copy_assign(&self, _a0: Ptr<ImplicitT>) -> Ptr<ImplicitT>;
}
impl ImplicitTImpl for Ptr<ImplicitT> {
    fn copy_assign(&self, _a0: Ptr<ImplicitT>) -> Ptr<ImplicitT> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
