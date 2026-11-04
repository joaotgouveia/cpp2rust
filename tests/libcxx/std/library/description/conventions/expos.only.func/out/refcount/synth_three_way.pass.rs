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
pub fn synth_three_way_3(t: Ptr<i32>, u: Ptr<i32>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_4(t: Ptr<f64>, u: Ptr<f64>) -> () {
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
pub fn synth_three_way_17(t: Ptr<i32>, u: Ptr<u32>) -> () {
    return unimplemented!();
}
pub fn synth_three_way_18(t: Ptr<i64>, u: Ptr<u8>) -> () {
    return unimplemented!();
}
pub fn operator_eq_19(_a0: Ptr<CustomEquality>, mut _a1: i32) -> bool {
    return true;
}
pub fn operator_lt_20(_a0: Ptr<CustomEquality>, mut _a1: i32) -> bool {
    return false;
}
pub fn operator_lt_21(mut _a0: i32, _a1: Ptr<CustomEquality>) -> bool {
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CustomEquality {}
impl CppMove for CustomEquality {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_22() -> bool {
    {
        assert!(
            ({
                let _t: Value<i32> = Rc::new(RefCell::new(1));
                let _u: Value<i32> = Rc::new(RefCell::new(1));
                synth_three_way_3(_t.as_pointer(), _u.as_pointer())
            }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
        );
        assert!(
            ({
                let _t: Value<i32> = Rc::new(RefCell::new(2));
                let _u: Value<i32> = Rc::new(RefCell::new(1));
                synth_three_way_3(_t.as_pointer(), _u.as_pointer())
            }) == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) })
        );
        assert!(
            ({
                let _t: Value<i32> = Rc::new(RefCell::new(1));
                let _u: Value<i32> = Rc::new(RefCell::new(2));
                synth_three_way_3(_t.as_pointer(), _u.as_pointer())
            }) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let nan: Value<f64> = Rc::new(RefCell::new(nan_f64));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        assert!(
            ({
                let _t: Value<StrongSpaceship_6> =
                    Rc::new(RefCell::new(StrongSpaceship_6 { value: 1 }));
                let _u: Value<StrongSpaceship_6> =
                    Rc::new(RefCell::new(StrongSpaceship_6 { value: 1 }));
                synth_three_way_5(_t.as_pointer(), _u.as_pointer())
            }) == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
        );
        assert!(
            ({
                let _t: Value<StrongSpaceship_6> =
                    Rc::new(RefCell::new(StrongSpaceship_6 { value: 2 }));
                let _u: Value<StrongSpaceship_6> =
                    Rc::new(RefCell::new(StrongSpaceship_6 { value: 1 }));
                synth_three_way_5(_t.as_pointer(), _u.as_pointer())
            }) == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) })
        );
        assert!(
            ({
                let _t: Value<StrongSpaceship_6> =
                    Rc::new(RefCell::new(StrongSpaceship_6 { value: 1 }));
                let _u: Value<StrongSpaceship_6> =
                    Rc::new(RefCell::new(StrongSpaceship_6 { value: 2 }));
                synth_three_way_5(_t.as_pointer(), _u.as_pointer())
            }) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) })
        );
        const _: () = assert!(true, "");;
    }
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
    }
    {
        let mut nan: f64 = nan_f64;
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
    }
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
    }
    {
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
    }
    {
        assert!(
            ({
                let _arg0: Value<CustomEquality> = Rc::new(RefCell::new(
                    ({
                        let _arg0: Value<CustomSpaceship_16> =
                            Rc::new(RefCell::new(<CustomSpaceship_16>::default()));
                        CustomSpaceship_16Impl::operator_cmp(
                            &Rc::new(RefCell::new(<CustomSpaceship_16>::default())).as_pointer(),
                            _arg0.as_pointer(),
                        )
                    }),
                ));
                operator_eq_19(_arg0.as_pointer(), 0)
            })
        );
        assert!(
            !({
                let _arg0: Value<CustomEquality> = Rc::new(RefCell::new(
                    ({
                        let _arg0: Value<CustomSpaceship_16> =
                            Rc::new(RefCell::new(<CustomSpaceship_16>::default()));
                        CustomSpaceship_16Impl::operator_cmp(
                            &Rc::new(RefCell::new(<CustomSpaceship_16>::default())).as_pointer(),
                            _arg0.as_pointer(),
                        )
                    }),
                ));
                operator_lt_20(_arg0.as_pointer(), 0)
            })
        );
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(!(false), "!can_synth_three_way<NoRelative>");;
    }
    {
        const _: () = assert!(!(false), "!can_synth_three_way<NoLessThan>");;
    }
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
        const _: () = assert!(true, "");;
        assert!(
            ({
                let _t: Value<i64> = Rc::new(RefCell::new((-1_i32 as i64)));
                let _u: Value<u8> = Rc::new(RefCell::new(0_u8));
                synth_three_way_18(_t.as_pointer(), _u.as_pointer())
            }) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) })
        );
        assert!(
            ({
                let _t: Value<i64> = Rc::new(RefCell::new((-1_i32 as i64)));
                let _u: Value<u8> = Rc::new(RefCell::new(0_u8));
                synth_three_way_18(_t.as_pointer(), _u.as_pointer())
            }) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) })
        );
        const _: () = assert!(true, "");;
    }
    {
        let nan: Value<f64> = Rc::new(RefCell::new(nan_f64));
        assert!(unimplemented!());
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct StrongSpaceship_6 {
    #[offset(0)]
    pub value: i32,
}
impl CppLess for StrongSpaceship_6 {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        StrongSpaceship_6Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for StrongSpaceship_6 {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        StrongSpaceship_6Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for StrongSpaceship_6 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct WeakSpaceship_8 {
    #[offset(0)]
    pub value: i32,
}
impl CppLess for WeakSpaceship_8 {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        WeakSpaceship_8Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for WeakSpaceship_8 {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        WeakSpaceship_8Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for WeakSpaceship_8 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PartialSpaceship_10 {
    #[offset(0)]
    pub value: f64,
}
impl CppLess for PartialSpaceship_10 {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PartialSpaceship_10Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for PartialSpaceship_10 {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PartialSpaceship_10Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for PartialSpaceship_10 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoSpaceship_12 {
    #[offset(0)]
    pub value: i32,
}
impl CppLess for NoSpaceship_12 {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        NoSpaceship_12Impl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for NoSpaceship_12 {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        NoSpaceship_12Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for NoSpaceship_12 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SpaceshipNoEquals_14 {}
impl CppLess for SpaceshipNoEquals_14 {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SpaceshipNoEquals_14Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for SpaceshipNoEquals_14 {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SpaceshipNoEquals_14Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
impl CppMove for SpaceshipNoEquals_14 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CustomSpaceship_16 {}
impl CppMove for CustomSpaceship_16 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoRelative_23 {}
impl CppMove for NoRelative_23 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoLessThan_24 {}
impl CppMove for NoLessThan_24 {
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
    ({ test_22() });
    const _: () = assert!(true, "test()");;
    return 0;
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
    fn operator_eq(&self, _a0: Ptr<NoLessThan_24>) -> bool {
        unimplemented!()
    }
    fn operator_gt(&self, _a0: Ptr<NoLessThan_24>) -> bool {
        unimplemented!()
    }
    fn operator_ge(&self, _a0: Ptr<NoLessThan_24>) -> bool {
        unimplemented!()
    }
    fn operator_le(&self, _a0: Ptr<NoLessThan_24>) -> bool {
        unimplemented!()
    }
}
impl NoLessThan_24Impl for Ptr<NoLessThan_24> {}
pub trait NoRelative_23Impl {
    fn operator_eq(&self, _a0: Ptr<NoRelative_23>) -> bool {
        unimplemented!()
    }
}
impl NoRelative_23Impl for Ptr<NoRelative_23> {}
pub trait NoSpaceship_12Impl {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_12>) -> bool;
    fn operator_lt(&self, other: Ptr<NoSpaceship_12>) -> bool;
}
impl NoSpaceship_12Impl for Ptr<NoSpaceship_12> {
    fn operator_eq(&self, _a0: Ptr<NoSpaceship_12>) -> bool {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_lt(&self, other: Ptr<NoSpaceship_12>) -> bool {
        return ({ (*self).with(|__s| __s.value) } < { other.with(|__s| __s.value) });
    }
}
pub trait PartialSpaceship_10Impl {
    fn operator_eq(&self, _a0: Ptr<PartialSpaceship_10>) -> bool;
    fn operator_cmp(&self, other: Ptr<PartialSpaceship_10>) -> ();
}
impl PartialSpaceship_10Impl for Ptr<PartialSpaceship_10> {
    fn operator_eq(&self, _a0: Ptr<PartialSpaceship_10>) -> bool {
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
    fn operator_cmp(&self, _a0: Ptr<SpaceshipNoEquals_14>) -> std::cmp::Ordering;
}
impl SpaceshipNoEquals_14Impl for Ptr<SpaceshipNoEquals_14> {
    fn operator_cmp(&self, _a0: Ptr<SpaceshipNoEquals_14>) -> std::cmp::Ordering {
        return ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) });
    }
}
pub trait StrongSpaceship_6Impl {
    fn operator_eq(&self, _a0: Ptr<StrongSpaceship_6>) -> bool;
    fn operator_cmp(&self, other: Ptr<StrongSpaceship_6>) -> std::cmp::Ordering;
}
impl StrongSpaceship_6Impl for Ptr<StrongSpaceship_6> {
    fn operator_eq(&self, _a0: Ptr<StrongSpaceship_6>) -> bool {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_cmp(&self, other: Ptr<StrongSpaceship_6>) -> std::cmp::Ordering {
        return std::cmp::Ord::cmp(
            &((*self).with(|__s| __s.value)),
            &(other.with(|__s| __s.value)),
        );
    }
}
pub trait WeakSpaceship_8Impl {
    fn operator_eq(&self, _a0: Ptr<WeakSpaceship_8>) -> bool;
    fn operator_cmp(&self, other: Ptr<WeakSpaceship_8>) -> ();
}
impl WeakSpaceship_8Impl for Ptr<WeakSpaceship_8> {
    fn operator_eq(&self, _a0: Ptr<WeakSpaceship_8>) -> bool {
        return ({ (*self).with(|__s| __s.value) } == { _a0.with(|__s| __s.value) });
    }
    fn operator_cmp(&self, other: Ptr<WeakSpaceship_8>) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
