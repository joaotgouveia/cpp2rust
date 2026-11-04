extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
impl CppMove for test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
impl CppMove for test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_weak_ordering__std_weak_ordering_ {}
impl CppMove for test_macros_detail_is_same_std_weak_ordering__std_weak_ordering_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_3() -> bool {
    {
        const _: () = assert!(true, "");;
        assert!((unimplemented!()) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }));
        assert!(
            (unimplemented!()) == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) })
        );
        assert!((unimplemented!()) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        const _: () = assert!(
            !(false),
            "!HasSpaceship<std::pair<int, int>, std::pair<std::string, int>>"
        );;
    }
    {
        const _: () = assert!(true, "");;
        assert!((unimplemented!()) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }));
        assert!(
            (unimplemented!()) == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) })
        );
        assert!((unimplemented!()) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) }));
    }
    {
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut nan: f64 = nan_f64;
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut nan: f64 = nan_f64;
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        const _: () = assert!(true, "HasEqual<std::pair<int, NoRelative>>");;
        const _: () = assert!(!(false), "!HasLess<std::pair<int, NoRelative>>");;
        const _: () = assert!(!(false), "!HasSpaceship<std::pair<int, NoRelative>>");;
    }
    {
        const _: () = assert!(true, "HasEqual<std::pair<int, NoLessThan>>");;
        const _: () = assert!(!(false), "!HasLess<std::pair<int, NoLessThan>>");;
        const _: () = assert!(!(false), "!HasSpaceship<std::pair<int, NoLessThan>>");;
    }
    {
        {
            let mut nan: f64 = nan_f64;
            assert!(unimplemented!());
        }
        {
            let mut nan: f64 = nan_f64;
            assert!(unimplemented!());
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoSpaceship_4 {
    #[offset(0)]
    pub value: i32,
}
impl CppLess for NoSpaceship_4 {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        NoSpaceship_4Impl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for NoSpaceship_4 {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        NoSpaceship_4Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for NoSpaceship_4 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoRelative_5 {}
impl CppMove for NoRelative_5 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoLessThan_6 {}
impl CppMove for NoLessThan_6 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait NoLessThan_6Impl {
    fn operator_eq(&self, _a0: Ptr<NoLessThan_6>) -> bool {
        unimplemented!()
    }
    fn operator_gt(&self, _a0: Ptr<NoLessThan_6>) -> bool {
        unimplemented!()
    }
}
impl NoLessThan_6Impl for Ptr<NoLessThan_6> {}
pub trait NoRelative_5Impl {
    fn operator_eq(&self, _a0: Ptr<NoRelative_5>) -> bool {
        unimplemented!()
    }
}
impl NoRelative_5Impl for Ptr<NoRelative_5> {}
pub trait NoSpaceship_4Impl {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_4>) -> bool;
    fn operator_lt(&self, other: Ptr<NoSpaceship_4>) -> bool;
}
impl NoSpaceship_4Impl for Ptr<NoSpaceship_4> {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_4>) -> bool {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_lt(&self, other: Ptr<NoSpaceship_4>) -> bool {
        return ({ (*self).with(|__s| __s.value) } < { other.with(|__s| __s.value) });
    }
}
pub fn __cpp2rust_init_globals() {}
