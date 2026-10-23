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
pub fn test_3() -> () {
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
    }
    {
        const _: () = assert!(
            !(false),
            "!HasSpaceship<std::pair<int, int>, std::pair<std::string, int>>"
        );;
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
    }
    {
        let mut nan: () = ({ std_numeric_limits_double_::quiet_NaN() });
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
        let mut nan: () = ({ std_numeric_limits_double_::quiet_NaN() });
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
            let mut nan: () = ({ std_numeric_limits_double_::quiet_NaN() });
            unimplemented!();
        }
        {
            let mut nan: () = ({ std_numeric_limits_double_::quiet_NaN() });
            unimplemented!();
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoSpaceship_4 {
    #[offset(0)]
    pub value: (),
}
impl std::cmp::Ord for NoSpaceship_4 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if NoSpaceship_4Impl::operator_lt(
                &Rc::new(RefCell::new(NoSpaceship_4 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_4 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if NoSpaceship_4Impl::operator_lt(
                &Rc::new(RefCell::new(NoSpaceship_4 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_4 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for NoSpaceship_4 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for NoSpaceship_4 {
    fn eq(&self, other: &Self) -> bool {
        {
            NoSpaceship_4Impl::operator_eq(
                &Rc::new(RefCell::new(NoSpaceship_4 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_4 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NoSpaceship_4 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoRelative_5 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoLessThan_6 {}
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
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait NoLessThan_6Impl {
    fn operator_eq(&self, _a0: Ptr<NoLessThan_6>) -> () {
        unimplemented!()
    }
    fn operator_gt(&self, _a0: Ptr<NoLessThan_6>) -> () {
        unimplemented!()
    }
}
impl NoLessThan_6Impl for Ptr<NoLessThan_6> {}
pub trait NoRelative_5Impl {
    fn operator_eq(&self, _a0: Ptr<NoRelative_5>) -> () {
        unimplemented!()
    }
}
impl NoRelative_5Impl for Ptr<NoRelative_5> {}
pub trait NoSpaceship_4Impl {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_4>) -> ();
    fn operator_lt(&self, other: Ptr<NoSpaceship_4>) -> ();
}
impl NoSpaceship_4Impl for Ptr<NoSpaceship_4> {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_4>) -> () {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_lt(&self, other: Ptr<NoSpaceship_4>) -> () {
        return ({ (*self).with(|__s| __s.value) } < { other.with(|__s| __s.value) });
    }
}
pub fn __cpp2rust_init_globals() {}
