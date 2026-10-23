extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct NothrowCopyAssignable {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct PotentiallyThrowingCopyAssignable {}
pub fn test_0() -> () {
    {
        let t0: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(2_()));
            let __tmp_1: Value<()> = Rc::new(RefCell::new(('a' as ())));
            std_pair_long__char_::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*t1.borrow_mut()) = (*t0.borrow()).clone();
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
        const _: () = assert!(true, "!std::is_assignable<T&, const P &>::value");;
    }
    {
        const _: () = assert!(
            true,
            "std::is_nothrow_assignable<Tuple&, Pair const&>::value"
        );;
        const _: () = assert!(true, "std::is_nothrow_assignable<Tuple&, Pair&>::value");;
        const _: () = assert!(
            true,
            "std::is_nothrow_assignable<Tuple&, Pair const&&>::value"
        );;
    }
    {
        const _: () = assert!(true, "std::is_assignable<Tuple&, Pair const&>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_assignable<Tuple&, Pair const&>::value"
        );;
        const _: () = assert!(true, "std::is_assignable<Tuple&, Pair&>::value");;
        const _: () = assert!(true, "!std::is_nothrow_assignable<Tuple&, Pair&>::value");;
        const _: () = assert!(true, "std::is_assignable<Tuple&, Pair const&&>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_assignable<Tuple&, Pair const&&>::value"
        );;
    }
    return 0_();
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
