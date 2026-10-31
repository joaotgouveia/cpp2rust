extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_weak_ordering__std_weak_ordering_ {}
pub unsafe fn synth_three_way_3(t: *const i32, u: *const i32) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_4(t: *const f64, u: *const f64) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_5(t: *const StrongSpaceship_6, u: *const StrongSpaceship_6) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_7(t: *const WeakSpaceship_8, u: *const WeakSpaceship_8) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_9(
    t: *const PartialSpaceship_10,
    u: *const PartialSpaceship_10,
) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_11(t: *const NoSpaceship_12, u: *const NoSpaceship_12) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_13(
    t: *const SpaceshipNoEquals_14,
    u: *const SpaceshipNoEquals_14,
) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_15(t: *const CustomSpaceship_16, u: *const CustomSpaceship_16) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_17(t: *const i32, u: *const u32) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_18(t: *const i64, u: *const u8) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_19(_a0: *const CustomEquality, mut _a1: i32) -> bool {
    return true;
}
pub unsafe fn operator_lt_20(_a0: *const CustomEquality, mut _a1: i32) -> bool {
    return false;
}
pub unsafe fn operator_lt_21(mut _a0: i32, _a1: *const CustomEquality) -> bool {
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CustomEquality {}
pub unsafe fn test_22() -> bool {
    {
        assert!(
            (unsafe {
                let mut _t: i32 = 1;
                let mut _u: i32 = 1;
                synth_three_way_3(&mut _t, &mut _u)
            }) == std::cmp::Ordering::Equal
        );
        assert!(
            (unsafe {
                let mut _t: i32 = 2;
                let mut _u: i32 = 1;
                synth_three_way_3(&mut _t, &mut _u)
            }) == std::cmp::Ordering::Greater
        );
        assert!(
            (unsafe {
                let mut _t: i32 = 1;
                let mut _u: i32 = 2;
                synth_three_way_3(&mut _t, &mut _u)
            }) == std::cmp::Ordering::Less
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let nan: f64 = unimplemented!();
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
            (unsafe {
                let mut _t: StrongSpaceship_6 = StrongSpaceship_6 { value: 1 };
                let mut _u: StrongSpaceship_6 = StrongSpaceship_6 { value: 1 };
                synth_three_way_5(&mut _t, &mut _u)
            }) == std::cmp::Ordering::Equal
        );
        assert!(
            (unsafe {
                let mut _t: StrongSpaceship_6 = StrongSpaceship_6 { value: 2 };
                let mut _u: StrongSpaceship_6 = StrongSpaceship_6 { value: 1 };
                synth_three_way_5(&mut _t, &mut _u)
            }) == std::cmp::Ordering::Greater
        );
        assert!(
            (unsafe {
                let mut _t: StrongSpaceship_6 = StrongSpaceship_6 { value: 1 };
                let mut _u: StrongSpaceship_6 = StrongSpaceship_6 { value: 2 };
                synth_three_way_5(&mut _t, &mut _u)
            }) == std::cmp::Ordering::Less
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
        let nan: f64 = unimplemented!();
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
            (unsafe {
                let mut _arg0: CustomEquality = (unsafe {
                    let mut _arg0: CustomSpaceship_16 = <CustomSpaceship_16>::default();
                    CustomSpaceship_16::operator_cmp(&<CustomSpaceship_16>::default(), &mut _arg0)
                });
                operator_eq_19(&mut _arg0, 0)
            })
        );
        assert!(
            !(unsafe {
                let mut _arg0: CustomEquality = (unsafe {
                    let mut _arg0: CustomSpaceship_16 = <CustomSpaceship_16>::default();
                    CustomSpaceship_16::operator_cmp(&<CustomSpaceship_16>::default(), &mut _arg0)
                });
                operator_lt_20(&mut _arg0, 0)
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
            (unsafe {
                let mut _t: i64 = (-1_i32 as i64);
                let mut _u: u8 = 0_u8;
                synth_three_way_18(&mut _t, &mut _u)
            }) == std::cmp::Ordering::Less
        );
        assert!(
            (unsafe {
                let mut _t: i64 = (-1_i32 as i64);
                let mut _u: u8 = 0_u8;
                synth_three_way_18(&mut _t, &mut _u)
            }) == std::cmp::Ordering::Less
        );
        const _: () = assert!(true, "");;
    }
    {
        let nan: f64 = unimplemented!();
        assert!(unimplemented!());
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StrongSpaceship_6 {
    pub value: i32,
}
impl StrongSpaceship_6 {
    pub unsafe fn operator_eq(&self, _a0: *const StrongSpaceship_6) -> bool {
        return (((*(self as *const StrongSpaceship_6)).value) == ((*_a0).value));
    }
    pub unsafe fn operator_cmp(&self, other: *const StrongSpaceship_6) -> std::cmp::Ordering {
        return std::cmp::Ord::cmp(&(self.value), &((*other).value));
    }
}
impl std::cmp::Ord for StrongSpaceship_6 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { StrongSpaceship_6::operator_cmp(self, other as *const StrongSpaceship_6) }
    }
}
impl std::cmp::PartialOrd for StrongSpaceship_6 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongSpaceship_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { StrongSpaceship_6::operator_eq(self, other as *const StrongSpaceship_6) }
    }
}
impl std::cmp::Eq for StrongSpaceship_6 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WeakSpaceship_8 {
    pub value: i32,
}
impl WeakSpaceship_8 {
    pub unsafe fn operator_eq(&self, _a0: *const WeakSpaceship_8) -> bool {
        return (((*(self as *const WeakSpaceship_8)).value) == ((*_a0).value));
    }
    pub unsafe fn operator_cmp(&self, other: *const WeakSpaceship_8) -> () {
        return unimplemented!();
    }
}
impl std::cmp::Ord for WeakSpaceship_8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { WeakSpaceship_8::operator_cmp(self, other as *const WeakSpaceship_8) }
    }
}
impl std::cmp::PartialOrd for WeakSpaceship_8 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakSpaceship_8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { WeakSpaceship_8::operator_eq(self, other as *const WeakSpaceship_8) }
    }
}
impl std::cmp::Eq for WeakSpaceship_8 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PartialSpaceship_10 {
    pub value: f64,
}
impl PartialSpaceship_10 {
    pub unsafe fn operator_eq(&self, _a0: *const PartialSpaceship_10) -> bool {
        return (((*(self as *const PartialSpaceship_10)).value) == ((*_a0).value));
    }
    pub unsafe fn operator_cmp(&self, other: *const PartialSpaceship_10) -> () {
        return std::cmp::Ord::cmp(&(self.value), &((*other).value));
    }
}
impl std::cmp::Ord for PartialSpaceship_10 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { PartialSpaceship_10::operator_cmp(self, other as *const PartialSpaceship_10) }
    }
}
impl std::cmp::PartialOrd for PartialSpaceship_10 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialSpaceship_10 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { PartialSpaceship_10::operator_eq(self, other as *const PartialSpaceship_10) }
    }
}
impl std::cmp::Eq for PartialSpaceship_10 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoSpaceship_12 {
    pub value: i32,
}
impl NoSpaceship_12 {
    pub unsafe fn operator_eq(&self, _a0: *const NoSpaceship_12) -> bool {
        return (((*(self as *const NoSpaceship_12)).value) == ((*_a0).value));
    }
    pub unsafe fn operator_lt(&self, other: *const NoSpaceship_12) -> bool {
        return ((self.value) < ((*other).value));
    }
}
impl std::cmp::Ord for NoSpaceship_12 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if NoSpaceship_12::operator_lt(self, other as *const NoSpaceship_12) {
                std::cmp::Ordering::Less
            } else if NoSpaceship_12::operator_lt(other, self as *const NoSpaceship_12) {
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
        unsafe { NoSpaceship_12::operator_eq(self, other as *const NoSpaceship_12) }
    }
}
impl std::cmp::Eq for NoSpaceship_12 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SpaceshipNoEquals_14 {}
impl SpaceshipNoEquals_14 {
    pub unsafe fn operator_cmp(&self, _a0: *const SpaceshipNoEquals_14) -> std::cmp::Ordering {
        return std::cmp::Ordering::Equal;
    }
}
impl std::cmp::Ord for SpaceshipNoEquals_14 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { SpaceshipNoEquals_14::operator_cmp(self, other as *const SpaceshipNoEquals_14) }
    }
}
impl std::cmp::PartialOrd for SpaceshipNoEquals_14 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for SpaceshipNoEquals_14 {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            SpaceshipNoEquals_14::operator_cmp(self, other as *const SpaceshipNoEquals_14)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for SpaceshipNoEquals_14 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CustomSpaceship_16 {}
impl CustomSpaceship_16 {
    pub unsafe fn operator_cmp(&self, _a0: *const CustomSpaceship_16) -> CustomEquality {
        return <CustomEquality>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoRelative_23 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoLessThan_24 {}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_22() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
