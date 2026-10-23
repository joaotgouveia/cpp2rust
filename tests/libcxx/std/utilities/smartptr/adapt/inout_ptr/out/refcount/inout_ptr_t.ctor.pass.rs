extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_convertible_imp_0(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_1(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_2() -> () {
    return ({ test_convertible_imp_0(0_()) });
}
pub fn test_convertible_3() -> () {
    return ({ test_convertible_imp_1(0_()) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct MoveOnlyDeleter_int_ {
    #[offset(0)]
    pub wasMoveInitialized: (),
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
    pub ptr: (),
}
impl ConstructiblePtr_int_ {
    pub fn move_from(_a0: Ptr<ConstructiblePtr_int_>) -> Self {
        Self {
            ptr: unimplemented!(),
        }
    }
}
const _: () = assert!(
    true,
    "std::is_same_v<std::__pointer_of_t< ConstructiblePtr<int>>, int* >"
);
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
    pub ptr: (),
}
impl ResettablePtr_int_ {
    pub fn move_from(_a0: Ptr<ResettablePtr_int_>) -> Self {
        Self {
            ptr: unimplemented!(),
        }
    }
}
const _: () = assert!(
    true,
    "std::is_same_v<std::__pointer_of_t< ResettablePtr<int>>, int* >"
);
const _: () = assert!(true, "std::is_constructible_v< ResettablePtr<int>, int* >");
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(8)]
pub struct NonConstructiblePtr_int_ {}
impl NonConstructiblePtr_int_ {}
const _: () = assert!(
    true,
    "std::is_same_v<std::__pointer_of_t< NonConstructiblePtr<int>>, int* >"
);
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let mut uPtr: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "!test_convertible<std::inout_ptr_t<std::unique_ptr<int>, int*>>()"
        );;
        unimplemented!();
    }
    {
        let deleter: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut p: Ptr<()>| {
                p.delete();
            })));
        let mut uPtr: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "!test_convertible<std::inout_ptr_t<std::unique_ptr<int, decltype(deleter)>, int*>>()"
        );;
        unimplemented!();
    }
    {
        let uPtr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        std_inout_ptr_t_std_unique_ptr_int__MoveOnlyDeleter_int____int_ptr__MoveOnlyDeleter_int__ :: new ( { uPtr .as_pointer()  } , { <MoveOnlyDeleter_int_  >::default()  } , )  ;
        unimplemented!();
    }
    return 0_();
}
pub trait ConstructiblePtr_int_Impl {
    fn release(&self) {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<ConstructiblePtr_int_>) -> Ptr<ConstructiblePtr_int_>;
}
impl ConstructiblePtr_int_Impl for Ptr<ConstructiblePtr_int_> {
    fn move_assign(&self, _a0: Ptr<ConstructiblePtr_int_>) -> Ptr<ConstructiblePtr_int_> {
        unimplemented!();
        return (*self).clone();
    }
}
pub trait MoveOnlyDeleter_int_Impl {
    fn move_assign(&self, _a0: Ptr<MoveOnlyDeleter_int_>) -> Ptr<MoveOnlyDeleter_int_>;
    fn operator_call(&self, p: Ptr<()>);
}
impl MoveOnlyDeleter_int_Impl for Ptr<MoveOnlyDeleter_int_> {
    fn move_assign(&self, _a0: Ptr<MoveOnlyDeleter_int_>) -> Ptr<MoveOnlyDeleter_int_> {
        field!((*self), wasMoveInitialized)
            .write({ { (*_a0.upgrade().deref()).wasMoveInitialized } });
        return (*self).clone();
    }
    fn operator_call(&self, mut p: Ptr<()>) {
        p.delete();
    }
}
pub trait NonConstructiblePtr_int_Impl {
    fn reset(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
}
impl NonConstructiblePtr_int_Impl for Ptr<NonConstructiblePtr_int_> {}
pub trait ResettablePtr_int_Impl {
    fn reset_4(&self) {
        unimplemented!()
    }
    fn reset_5(&self, mut p: Ptr<()>, mut _a1: ResetArg) {
        unimplemented!()
    }
    fn release(&self) {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<ResettablePtr_int_>) -> Ptr<ResettablePtr_int_>;
}
impl ResettablePtr_int_Impl for Ptr<ResettablePtr_int_> {
    fn move_assign(&self, _a0: Ptr<ResettablePtr_int_>) -> Ptr<ResettablePtr_int_> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
