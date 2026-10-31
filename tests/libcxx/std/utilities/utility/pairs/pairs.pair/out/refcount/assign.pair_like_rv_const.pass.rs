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
        {
            let mut x: i32 = 91;
            let mut y: i32 = 92;
            let a: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2])));
            let p: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> =
                Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = (unimplemented!());
            assert!(({ (result).clone() } == { (p.as_pointer()) }));
            assert!((x == 1));
            assert!((y == 2));
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
            let mut x: i32 = 91;
            let mut y: i32 = 92;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> =
                Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = (unimplemented!());
            assert!(({ (result).clone() } == { (p.as_pointer()) }));
            assert!((x == 1));
            assert!((y == 2));
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
            let data: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<(Value<ConstAssignable_1>, Value<ConstAssignable_1>)> =
                Rc::new(RefCell::new(unimplemented!()));
            let result: Ptr<(Value<ConstAssignable_1>, Value<ConstAssignable_1>)> =
                (unimplemented!());
            assert!(({ (result).clone() } == { (p.as_pointer()) }));
            assert!(
                ({ { (*(*p.borrow()).0.borrow()).ptr.clone() } } == {
                    (data.as_pointer() as Ptr<i32>)
                })
            );
            assert!(
                ({ { (*(*p.borrow()).1.borrow()).ptr.clone() } } == {
                    (data.as_pointer() as Ptr<i32>).offset((5) as isize)
                })
            );
        }
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<(Value<ConstAssignable_2>, Value<ConstAssignable_2>)> =
            Rc::new(RefCell::new(unimplemented!()));
        let result: Ptr<(Value<ConstAssignable_2>, Value<ConstAssignable_2>)> = (unimplemented!());
        assert!(({ (result).clone() } == { (p.as_pointer()) }));
        assert!(({ (*(*p.borrow()).0.borrow()).val } == 1));
        assert!(({ (*(*p.borrow()).1.borrow()).val } == 2));
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
        let p: Value<(Value<NoCopy_3>, Value<NoCopy_3>)> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(Ptr< i32  >  , new_1, value)]
#[byte_size(8)]
pub struct ConstAssignable_1 {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
}
impl ConstAssignable_1 {
    pub fn new_1(mut p: Ptr<i32>) -> Self {
        Self { ptr: (p).clone() }
    }
}
impl Default for ConstAssignable_1 {
    fn default() -> Self {
        {
            ConstAssignable_1 {
                ptr: Ptr::<i32>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConstAssignable_2 {
    #[offset(0)]
    pub val: i32,
}
impl Default for ConstAssignable_2 {
    fn default() -> Self {
        { ConstAssignable_2 { val: 0 } }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
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
    fn operator_assign_2(&self, v: i32) -> Ptr<ConstAssignable_2>;
}
impl ConstAssignable_2Impl for Ptr<ConstAssignable_2> {
    fn operator_assign_2(&self, mut v: i32) -> Ptr<ConstAssignable_2> {
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
