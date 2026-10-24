extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonAssignable {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyAssignable {}
const _: () = assert!(true, "std::is_copy_assignable<CopyAssignable>::value");
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveAssignable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct NothrowCopyAssignable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct PotentiallyThrowingCopyAssignable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyAssignableInt {}
pub fn test_0() -> () {
    {
        let t0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        unimplemented!();
    }
    {
        let t0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let y: Value<()> = Rc::new(RefCell::new(100_()));
        let mut x2: () = -1_();
        let y2: Value<()> = Rc::new(RefCell::new(500_()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t.borrow_mut()) = (*t2.borrow()).clone();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P const>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P&>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<T>::value");;
        const _: () = assert!(true, "!std::is_nothrow_copy_assignable<T>::value");;
    }
    return 0_();
}
pub trait CopyAssignableIntImpl {
    fn operator_assign_1(&self, _a0: Ptr<()>) -> Ptr<CopyAssignableInt>;
}
impl CopyAssignableIntImpl for Ptr<CopyAssignableInt> {
    fn operator_assign_1(&self, _a0: Ptr<()>) -> Ptr<CopyAssignableInt> {
        return (*self).clone();
    }
}
pub trait MoveAssignableImpl {
    fn move_assign(&self, _a0: Ptr<MoveAssignable>) -> Ptr<MoveAssignable>;
}
impl MoveAssignableImpl for Ptr<MoveAssignable> {
    fn move_assign(&self, _a0: Ptr<MoveAssignable>) -> Ptr<MoveAssignable> {
        return (*self).clone();
    }
}
pub trait NothrowCopyAssignableImpl {
    fn copy_assign(&self, _a0: Ptr<NothrowCopyAssignable>) -> Ptr<NothrowCopyAssignable>;
}
impl NothrowCopyAssignableImpl for Ptr<NothrowCopyAssignable> {
    fn copy_assign(&self, _a0: Ptr<NothrowCopyAssignable>) -> Ptr<NothrowCopyAssignable> {
        return (*self).clone();
    }
}
pub trait PotentiallyThrowingCopyAssignableImpl {
    fn copy_assign(
        &self,
        _a0: Ptr<PotentiallyThrowingCopyAssignable>,
    ) -> Ptr<PotentiallyThrowingCopyAssignable>;
}
impl PotentiallyThrowingCopyAssignableImpl for Ptr<PotentiallyThrowingCopyAssignable> {
    fn copy_assign(
        &self,
        _a0: Ptr<PotentiallyThrowingCopyAssignable>,
    ) -> Ptr<PotentiallyThrowingCopyAssignable> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
