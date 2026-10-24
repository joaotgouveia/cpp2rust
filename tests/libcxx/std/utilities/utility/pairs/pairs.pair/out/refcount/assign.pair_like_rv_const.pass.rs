extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> () {
    {
        {
            let mut x: () = 91_();
            let mut y: () = 92_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<()> = (unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int&, int&> const&, std::array<int, 1>>"
            );;
            const _: () = assert!(
                true,
                "std::is_assignable_v<std::pair<int&, int&> const&, std::array<int, 2>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int&, int&> const&, std::array<int, 3>>"
            );;
        }
        {
            let mut x: () = 91_();
            let mut y: () = 92_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<()> = (unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int&, int&> const&, std::tuple<int>>"
            );;
            const _: () = assert!(
                true,
                "std::is_assignable_v<std::pair<int&, int&> const&, std::tuple<int, int>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int&, int&> const&, std::tuple<int, int, int>>"
            );;
        }
        {
            let data: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<()> = (unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let result: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::pair<ConstAssignable, ConstAssignable> const&, std::tuple<void*, int>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::pair<ConstAssignable, ConstAssignable> const&, std::tuple<int, void*>>"
        );;
        const _: () = assert!(
            true,
            "std::is_assignable_v<std::pair<ConstAssignable, ConstAssignable> const&, std::tuple<int, int>>"
        );;
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConstAssignable_1 {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl ConstAssignable_1 {
    pub fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr: (p).clone() }
    }
}
impl Default for ConstAssignable_1 {
    fn default() -> Self {
        {
            ConstAssignable_1 {
                ptr: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConstAssignable_2 {
    #[offset(0)]
    pub val: (),
}
impl Default for ConstAssignable_2 {
    fn default() -> Self {
        { ConstAssignable_2 { val: 0_() } }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct NoCopy_3 {}
impl NoCopy_3 {
    pub fn move_from(_a0: Ptr<NoCopy_3>) -> Self {
        Self {}
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait ConstAssignable_1Impl {
    fn operator_assign_3(&self, other: Ptr<ConstAssignable_1>) -> Ptr<ConstAssignable_1>;
}
impl ConstAssignable_1Impl for Ptr<ConstAssignable_1> {
    fn operator_assign_3(&self, other: Ptr<ConstAssignable_1>) -> Ptr<ConstAssignable_1> {
        field!((*self), ptr).write({ other.with(|__s| __s.ptr.clone()) });
        return (*self).clone();
    }
}
pub trait ConstAssignable_2Impl {
    fn operator_assign_2(&self, v: ()) -> Ptr<ConstAssignable_2>;
}
impl ConstAssignable_2Impl for Ptr<ConstAssignable_2> {
    fn operator_assign_2(&self, mut v: ()) -> Ptr<ConstAssignable_2> {
        field!((*self), val).write(v);
        return (*self).clone();
    }
}
pub trait NoCopy_3Impl {
    fn move_assign(&self, _a0: Ptr<NoCopy_3>) -> Ptr<NoCopy_3>;
}
impl NoCopy_3Impl for Ptr<NoCopy_3> {
    fn move_assign(&self, _a0: Ptr<NoCopy_3>) -> Ptr<NoCopy_3> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
