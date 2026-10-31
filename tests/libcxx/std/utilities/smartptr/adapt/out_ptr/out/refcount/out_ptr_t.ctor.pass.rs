extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_convertible_imp_0(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_1(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_2() -> bool {
    return ({ test_convertible_imp_0(0_i64) });
}
pub fn test_convertible_3() -> bool {
    return ({ test_convertible_imp_1(0_i64) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct MoveOnlyDeleter_int_ {
    #[offset(0)]
    pub wasMoveInitialized: bool,
}
impl MoveOnlyDeleter_int_ {
    pub fn move_from(_a0: Ptr<MoveOnlyDeleter_int_>) -> Self {
        Self {
            wasMoveInitialized: true,
        }
    }
}
impl Default for MoveOnlyDeleter_int_ {
    fn default() -> Self {
        {
            MoveOnlyDeleter_int_ {
                wasMoveInitialized: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct ConstructiblePtr_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Option<Value<i32>>,
}
impl ConstructiblePtr_int_ {
    pub fn move_from(_a0: Ptr<ConstructiblePtr_int_>) -> Self {
        Self {
            ptr: field!(_a0, ptr).with_mut(|__v: &mut Option<Value<i32>>| __v.take()),
        }
    }
}
const _: () = assert!(true, "");
const _: () = assert!(
    true,
    "std::is_constructible_v< ConstructiblePtr<int>, int* >"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ResetArg {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct ResettablePtr_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Option<Value<i32>>,
}
impl ResettablePtr_int_ {
    pub fn move_from(_a0: Ptr<ResettablePtr_int_>) -> Self {
        Self {
            ptr: field!(_a0, ptr).with_mut(|__v: &mut Option<Value<i32>>| __v.take()),
        }
    }
}
const _: () = assert!(true, "");
const _: () = assert!(true, "std::is_constructible_v< ResettablePtr<int>, int* >");
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(8)]
pub struct NonConstructiblePtr_int_ {}
impl NonConstructiblePtr_int_ {}
const _: () = assert!(true, "");
const _: () = assert!(
    true,
    "!std::is_constructible_v< NonConstructiblePtr<int>, int* >"
);
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
        let mut uPtr: Option<Value<i32>> = None;
        unimplemented!();
        const _: () = assert!(
            true,
            "!test_convertible<std::out_ptr_t<std::unique_ptr<int>, int*>>()"
        );;
        assert!(uPtr.is_none());
    }
    {
        let uPtr: Value<Option<Value<i32>>> = Rc::new(RefCell::new(None));
        unimplemented!();
        const _: () = assert!(
            true,
            "!test_convertible<std::out_ptr_t<decltype(uPtr), int*, std::default_delete<int>>>()"
        );;
        assert!((*uPtr.borrow()).is_none());
    }
    {
        let uPtr: Value<Option<Value<i32>>> = Rc::new(RefCell::new(None));
        unimplemented!();
        assert!((*uPtr.borrow()).is_none());
    }
    return 0;
}
pub trait ConstructiblePtr_int_Impl {
    fn release(&self) {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<ConstructiblePtr_int_>) -> Ptr<ConstructiblePtr_int_>;
}
impl ConstructiblePtr_int_Impl for Ptr<ConstructiblePtr_int_> {
    fn move_assign(&self, _a0: Ptr<ConstructiblePtr_int_>) -> Ptr<ConstructiblePtr_int_> {
        field_ptr!((*self), ptr)
            .write(field!(_a0, ptr).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
}
pub trait MoveOnlyDeleter_int_Impl {
    fn move_assign(&self, _a0: Ptr<MoveOnlyDeleter_int_>) -> Ptr<MoveOnlyDeleter_int_>;
    fn operator_call(&self, p: Ptr<i32>);
}
impl MoveOnlyDeleter_int_Impl for Ptr<MoveOnlyDeleter_int_> {
    fn move_assign(&self, _a0: Ptr<MoveOnlyDeleter_int_>) -> Ptr<MoveOnlyDeleter_int_> {
        field!((*self), wasMoveInitialized)
            .write({ { (*_a0.upgrade().deref()).wasMoveInitialized } });
        return (*self).clone();
    }
    fn operator_call(&self, mut p: Ptr<i32>) {
        p.delete();
    }
}
pub trait NonConstructiblePtr_int_Impl {
    fn reset(&self, mut p: Ptr<i32>) {
        unimplemented!()
    }
}
impl NonConstructiblePtr_int_Impl for Ptr<NonConstructiblePtr_int_> {}
pub trait ResettablePtr_int_Impl {
    fn reset_4(&self) {
        unimplemented!()
    }
    fn reset_5(&self, mut p: Ptr<i32>, mut _a1: ResetArg) {
        unimplemented!()
    }
    fn release(&self) {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<ResettablePtr_int_>) -> Ptr<ResettablePtr_int_>;
}
impl ResettablePtr_int_Impl for Ptr<ResettablePtr_int_> {
    fn move_assign(&self, _a0: Ptr<ResettablePtr_int_>) -> Ptr<ResettablePtr_int_> {
        field_ptr!((*self), ptr)
            .write(field!(_a0, ptr).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
