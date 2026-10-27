extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_convertible_imp_0(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_1(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_2(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_3(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_4(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_5(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_6(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_7(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_8(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_9(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_10() -> () {
    return ({ test_convertible_imp_0(0_()) });
}
pub fn test_convertible_11() -> () {
    return ({ test_convertible_imp_5(0_()) });
}
pub fn test_convertible_12() -> () {
    return ({ test_convertible_imp_1(0_()) });
}
pub fn test_convertible_13() -> () {
    return ({ test_convertible_imp_6(0_()) });
}
pub fn test_convertible_14() -> () {
    return ({ test_convertible_imp_2(0_()) });
}
pub fn test_convertible_15() -> () {
    return ({ test_convertible_imp_7(0_()) });
}
pub fn test_convertible_16() -> () {
    return ({ test_convertible_imp_3(0_()) });
}
pub fn test_convertible_17() -> () {
    return ({ test_convertible_imp_8(0_()) });
}
pub fn test_convertible_18() -> () {
    return ({ test_convertible_imp_4(0_()) });
}
pub fn test_convertible_19() -> () {
    return ({ test_convertible_imp_9(0_()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ImplicitDefault {}
impl ImplicitDefault {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for ImplicitDefault {
    fn default() -> Self {
        { ImplicitDefault::new() }
    }
}
const _: () = assert!(true, "test_convertible<ImplicitDefault>()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct ExplicitDefault {}
impl ExplicitDefault {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for ExplicitDefault {
    fn default() -> Self {
        { ExplicitDefault::new() }
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitDefault>()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ImplicitInt {}
impl ImplicitInt {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
const _: () = assert!(true, "test_convertible<ImplicitInt, int>()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitInt {}
impl ExplicitInt {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitInt, int>()");
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct ImplicitCopy {}
impl ImplicitCopy {
    pub fn copy_from(_a0: Ptr<ImplicitCopy>) -> Self {
        Self {}
    }
}
impl Clone for ImplicitCopy {
    fn clone(&self) -> Self {
        let __src: Value<ImplicitCopy> = Rc::new(RefCell::new(ImplicitCopy {}));
        ImplicitCopy::copy_from(__src.as_pointer())
    }
}
const _: () = assert!(true, "test_convertible<ImplicitCopy, ImplicitCopy>()");
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct ExplicitCopy {}
impl ExplicitCopy {
    pub fn copy_from(_a0: Ptr<ExplicitCopy>) -> Self {
        Self {}
    }
}
impl Clone for ExplicitCopy {
    fn clone(&self) -> Self {
        let __src: Value<ExplicitCopy> = Rc::new(RefCell::new(ExplicitCopy {}));
        ExplicitCopy::copy_from(__src.as_pointer())
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitCopy, ExplicitCopy>()");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct ImplicitMove {}
impl ImplicitMove {
    pub fn move_from(_a0: Ptr<ImplicitMove>) -> Self {
        Self {}
    }
}
const _: () = assert!(true, "test_convertible<ImplicitMove, ImplicitMove>()");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct ExplicitMove {}
impl ExplicitMove {
    pub fn move_from(_a0: Ptr<ExplicitMove>) -> Self {
        Self {}
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitMove, ExplicitMove>()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ImplicitArgs {}
impl ImplicitArgs {
    pub fn new(mut _a0: (), mut _a1: (), mut _a2: ()) -> Self {
        Self {}
    }
}
const _: () = assert!(true, "test_convertible<ImplicitArgs, int, int, int>()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitArgs {}
impl ExplicitArgs {
    pub fn new(mut _a0: (), mut _a1: (), mut _a2: ()) -> Self {
        Self {}
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitArgs, int, int, int>()");
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
    return 0_();
}
pub trait ExplicitCopyImpl {
    fn copy_assign(&self, _a0: Ptr<ExplicitCopy>) -> Ptr<ExplicitCopy>;
}
impl ExplicitCopyImpl for Ptr<ExplicitCopy> {
    fn copy_assign(&self, _a0: Ptr<ExplicitCopy>) -> Ptr<ExplicitCopy> {
        return (*self).clone();
    }
}
pub trait ImplicitCopyImpl {
    fn copy_assign(&self, _a0: Ptr<ImplicitCopy>) -> Ptr<ImplicitCopy>;
}
impl ImplicitCopyImpl for Ptr<ImplicitCopy> {
    fn copy_assign(&self, _a0: Ptr<ImplicitCopy>) -> Ptr<ImplicitCopy> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
