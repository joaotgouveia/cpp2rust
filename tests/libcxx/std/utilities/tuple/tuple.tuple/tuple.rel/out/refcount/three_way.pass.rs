extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_weak_ordering__std_weak_ordering_ {}
pub fn operator_eq_3(_a0: Ptr<CustomEquality>, mut _a1: ()) -> () {
    return true;
}
pub fn operator_lt_4(_a0: Ptr<CustomEquality>, mut _a1: ()) -> () {
    return false;
}
pub fn operator_lt_5(mut _a0: (), _a1: Ptr<CustomEquality>) -> () {
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CustomEquality {}
pub fn test_6() -> () {
    {
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut nan: () = unimplemented!();
        unimplemented!();
    }
    {
        let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
    }
    {
        {
            let mut nan: () = unimplemented!();
            const _: () = assert!(true, "");;
            unimplemented!();
        }
        {
            let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(true, "");;
            unimplemented!();
            unimplemented!();
        }
        {
            let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(true, "");;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct WeakSpaceship_7 {}
impl std::cmp::Ord for WeakSpaceship_7 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            WeakSpaceship_7Impl::operator_cmp(
                &Rc::new(RefCell::new(WeakSpaceship_7 {})).as_pointer(),
                Rc::new(RefCell::new(WeakSpaceship_7 {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for WeakSpaceship_7 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakSpaceship_7 {
    fn eq(&self, other: &Self) -> bool {
        {
            WeakSpaceship_7Impl::operator_eq(
                &Rc::new(RefCell::new(WeakSpaceship_7 {})).as_pointer(),
                Rc::new(RefCell::new(WeakSpaceship_7 {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for WeakSpaceship_7 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoSpaceship_8 {}
impl std::cmp::Ord for NoSpaceship_8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if NoSpaceship_8Impl::operator_lt(
                &Rc::new(RefCell::new(NoSpaceship_8 {})).as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_8 {})).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if NoSpaceship_8Impl::operator_lt(
                &Rc::new(RefCell::new(NoSpaceship_8 {})).as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_8 {})).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for NoSpaceship_8 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for NoSpaceship_8 {
    fn eq(&self, other: &Self) -> bool {
        {
            NoSpaceship_8Impl::operator_eq(
                &Rc::new(RefCell::new(NoSpaceship_8 {})).as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_8 {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NoSpaceship_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SpaceshipNoEquals_9 {}
impl std::cmp::Ord for SpaceshipNoEquals_9 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            SpaceshipNoEquals_9Impl::operator_cmp(
                &Rc::new(RefCell::new(SpaceshipNoEquals_9 {})).as_pointer(),
                Rc::new(RefCell::new(SpaceshipNoEquals_9 {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for SpaceshipNoEquals_9 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for SpaceshipNoEquals_9 {
    fn eq(&self, other: &Self) -> bool {
        {
            SpaceshipNoEquals_9Impl::operator_cmp(
                &Rc::new(RefCell::new(SpaceshipNoEquals_9 {})).as_pointer(),
                Rc::new(RefCell::new(SpaceshipNoEquals_9 {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for SpaceshipNoEquals_9 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CustomSpaceship_10 {}
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
pub trait CustomSpaceship_10Impl {
    fn operator_cmp(&self, _a0: Ptr<CustomSpaceship_10>) -> CustomEquality;
}
impl CustomSpaceship_10Impl for Ptr<CustomSpaceship_10> {
    fn operator_cmp(&self, _a0: Ptr<CustomSpaceship_10>) -> CustomEquality {
        return <CustomEquality>::default();
    }
}
pub trait NoSpaceship_8Impl {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_8>) -> ();
    fn operator_lt(&self, _a0: Ptr<NoSpaceship_8>) -> ();
}
impl NoSpaceship_8Impl for Ptr<NoSpaceship_8> {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_8>) -> () {
        return true;
    }
    fn operator_lt(&self, _a0: Ptr<NoSpaceship_8>) -> () {
        return false;
    }
}
pub trait SpaceshipNoEquals_9Impl {
    fn operator_cmp(&self, _a0: Ptr<SpaceshipNoEquals_9>) -> ();
    fn operator_lt(&self, _a0: Ptr<SpaceshipNoEquals_9>) -> ();
}
impl SpaceshipNoEquals_9Impl for Ptr<SpaceshipNoEquals_9> {
    fn operator_cmp(&self, _a0: Ptr<SpaceshipNoEquals_9>) -> () {
        return unimplemented!();
    }
    fn operator_lt(&self, _a0: Ptr<SpaceshipNoEquals_9>) -> () {
        return false;
    }
}
pub trait WeakSpaceship_7Impl {
    fn operator_eq(&self, _a0: Ptr<WeakSpaceship_7>) -> ();
    fn operator_cmp(&self, _a0: Ptr<WeakSpaceship_7>) -> ();
}
impl WeakSpaceship_7Impl for Ptr<WeakSpaceship_7> {
    fn operator_eq(&self, _a0: Ptr<WeakSpaceship_7>) -> () {
        return true;
    }
    fn operator_cmp(&self, _a0: Ptr<WeakSpaceship_7>) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
