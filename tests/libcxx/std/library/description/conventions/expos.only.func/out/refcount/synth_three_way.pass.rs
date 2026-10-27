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
pub fn synth_three_way_3(t: Ptr<()>, u: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_4(t: Ptr<()>, u: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_5(t: Ptr<StrongSpaceship_6>, u: Ptr<StrongSpaceship_6>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_7(t: Ptr<WeakSpaceship_8>, u: Ptr<WeakSpaceship_8>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_9(t: Ptr<PartialSpaceship_10>, u: Ptr<PartialSpaceship_10>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_11(t: Ptr<NoSpaceship_12>, u: Ptr<NoSpaceship_12>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_13(t: Ptr<SpaceshipNoEquals_14>, u: Ptr<SpaceshipNoEquals_14>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_15(t: Ptr<CustomSpaceship_16>, u: Ptr<CustomSpaceship_16>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_17(t: Ptr<()>, u: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_18(t: Ptr<()>, u: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn operator_eq_19(_a0: Ptr<CustomEquality>, mut _a1: ()) -> () {
    return true;
}
pub fn operator_lt_20(_a0: Ptr<CustomEquality>, mut _a1: ()) -> () {
    return false;
}
pub fn operator_lt_21(mut _a0: (), _a1: Ptr<CustomEquality>) -> () {
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CustomEquality {}
pub fn test_22() -> () {
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut nan: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(!(false), "!can_synth_three_way<NoRelative>");;
    }
    {
        const _: () = assert!(!(false), "!can_synth_three_way<NoLessThan>");;
    }
    {
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct StrongSpaceship_6 {
    #[offset(0)]
    pub value: (),
}
impl std::cmp::Ord for StrongSpaceship_6 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            StrongSpaceship_6Impl::operator_cmp(
                &Rc::new(RefCell::new(StrongSpaceship_6 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(StrongSpaceship_6 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for StrongSpaceship_6 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongSpaceship_6 {
    fn eq(&self, other: &Self) -> bool {
        {
            StrongSpaceship_6Impl::operator_eq(
                &Rc::new(RefCell::new(StrongSpaceship_6 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(StrongSpaceship_6 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for StrongSpaceship_6 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct WeakSpaceship_8 {
    #[offset(0)]
    pub value: (),
}
impl std::cmp::Ord for WeakSpaceship_8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            WeakSpaceship_8Impl::operator_cmp(
                &Rc::new(RefCell::new(WeakSpaceship_8 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(WeakSpaceship_8 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for WeakSpaceship_8 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakSpaceship_8 {
    fn eq(&self, other: &Self) -> bool {
        {
            WeakSpaceship_8Impl::operator_eq(
                &Rc::new(RefCell::new(WeakSpaceship_8 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(WeakSpaceship_8 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for WeakSpaceship_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PartialSpaceship_10 {
    #[offset(0)]
    pub value: (),
}
impl std::cmp::Ord for PartialSpaceship_10 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            PartialSpaceship_10Impl::operator_cmp(
                &Rc::new(RefCell::new(PartialSpaceship_10 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(PartialSpaceship_10 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for PartialSpaceship_10 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialSpaceship_10 {
    fn eq(&self, other: &Self) -> bool {
        {
            PartialSpaceship_10Impl::operator_eq(
                &Rc::new(RefCell::new(PartialSpaceship_10 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(PartialSpaceship_10 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for PartialSpaceship_10 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoSpaceship_12 {
    #[offset(0)]
    pub value: (),
}
impl std::cmp::Ord for NoSpaceship_12 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if NoSpaceship_12Impl::operator_lt(
                &Rc::new(RefCell::new(NoSpaceship_12 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_12 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if NoSpaceship_12Impl::operator_lt(
                &Rc::new(RefCell::new(NoSpaceship_12 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_12 {
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
impl std::cmp::PartialOrd for NoSpaceship_12 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for NoSpaceship_12 {
    fn eq(&self, other: &Self) -> bool {
        {
            NoSpaceship_12Impl::operator_eq(
                &Rc::new(RefCell::new(NoSpaceship_12 {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NoSpaceship_12 {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NoSpaceship_12 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SpaceshipNoEquals_14 {}
impl std::cmp::Ord for SpaceshipNoEquals_14 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            SpaceshipNoEquals_14Impl::operator_cmp(
                &Rc::new(RefCell::new(SpaceshipNoEquals_14 {})).as_pointer(),
                Rc::new(RefCell::new(SpaceshipNoEquals_14 {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for SpaceshipNoEquals_14 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for SpaceshipNoEquals_14 {
    fn eq(&self, other: &Self) -> bool {
        {
            SpaceshipNoEquals_14Impl::operator_cmp(
                &Rc::new(RefCell::new(SpaceshipNoEquals_14 {})).as_pointer(),
                Rc::new(RefCell::new(SpaceshipNoEquals_14 {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for SpaceshipNoEquals_14 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CustomSpaceship_16 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoRelative_23 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoLessThan_24 {}
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
    ({ test_22() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait CustomSpaceship_16Impl {
    fn operator_cmp(&self, _a0: Ptr<CustomSpaceship_16>) -> CustomEquality;
}
impl CustomSpaceship_16Impl for Ptr<CustomSpaceship_16> {
    fn operator_cmp(&self, _a0: Ptr<CustomSpaceship_16>) -> CustomEquality {
        return <CustomEquality>::default();
    }
}
pub trait NoLessThan_24Impl {
    fn operator_eq(&self, _a0: Ptr<NoLessThan_24>) -> () {
        unimplemented!()
    }
    fn operator_gt(&self, _a0: Ptr<NoLessThan_24>) -> () {
        unimplemented!()
    }
    fn operator_ge(&self, _a0: Ptr<NoLessThan_24>) -> () {
        unimplemented!()
    }
    fn operator_le(&self, _a0: Ptr<NoLessThan_24>) -> () {
        unimplemented!()
    }
}
impl NoLessThan_24Impl for Ptr<NoLessThan_24> {}
pub trait NoRelative_23Impl {
    fn operator_eq(&self, _a0: Ptr<NoRelative_23>) -> () {
        unimplemented!()
    }
}
impl NoRelative_23Impl for Ptr<NoRelative_23> {}
pub trait NoSpaceship_12Impl {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_12>) -> ();
    fn operator_lt(&self, other: Ptr<NoSpaceship_12>) -> ();
}
impl NoSpaceship_12Impl for Ptr<NoSpaceship_12> {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_12>) -> () {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_lt(&self, other: Ptr<NoSpaceship_12>) -> () {
        return ({ (*self).with(|__s| __s.value) } < { other.with(|__s| __s.value) });
    }
}
pub trait PartialSpaceship_10Impl {
    fn operator_eq(&self, _a0: Ptr<PartialSpaceship_10>) -> ();
    fn operator_cmp(&self, other: Ptr<PartialSpaceship_10>) -> ();
}
impl PartialSpaceship_10Impl for Ptr<PartialSpaceship_10> {
    fn operator_eq(&self, _a0: Ptr<PartialSpaceship_10>) -> () {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_cmp(&self, other: Ptr<PartialSpaceship_10>) -> () {
        return std::cmp::Ord::cmp(
            &((*self).with(|__s| __s.value)),
            &(other.with(|__s| __s.value)),
        );
    }
}
pub trait SpaceshipNoEquals_14Impl {
    fn operator_cmp(&self, _a0: Ptr<SpaceshipNoEquals_14>) -> ();
}
impl SpaceshipNoEquals_14Impl for Ptr<SpaceshipNoEquals_14> {
    fn operator_cmp(&self, _a0: Ptr<SpaceshipNoEquals_14>) -> () {
        return unimplemented!();
    }
}
pub trait StrongSpaceship_6Impl {
    fn operator_eq(&self, _a0: Ptr<StrongSpaceship_6>) -> ();
    fn operator_cmp(&self, other: Ptr<StrongSpaceship_6>) -> ();
}
impl StrongSpaceship_6Impl for Ptr<StrongSpaceship_6> {
    fn operator_eq(&self, _a0: Ptr<StrongSpaceship_6>) -> () {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_cmp(&self, other: Ptr<StrongSpaceship_6>) -> () {
        return std::cmp::Ord::cmp(
            &((*self).with(|__s| __s.value)),
            &(other.with(|__s| __s.value)),
        );
    }
}
pub trait WeakSpaceship_8Impl {
    fn operator_eq(&self, _a0: Ptr<WeakSpaceship_8>) -> ();
    fn operator_cmp(&self, other: Ptr<WeakSpaceship_8>) -> ();
}
impl WeakSpaceship_8Impl for Ptr<WeakSpaceship_8> {
    fn operator_eq(&self, _a0: Ptr<WeakSpaceship_8>) -> () {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_cmp(&self, other: Ptr<WeakSpaceship_8>) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
