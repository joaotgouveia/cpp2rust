extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_weak_ordering__std_weak_ordering_ {}
pub unsafe fn synth_three_way_3(t: *const (), u: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_4(t: *const (), u: *const ()) -> () {
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
pub unsafe fn synth_three_way_17(t: *const (), u: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn synth_three_way_18(t: *const (), u: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_19(_a0: *const CustomEquality, mut _a1: ()) -> () {
    return true;
}
pub unsafe fn operator_lt_20(_a0: *const CustomEquality, mut _a1: ()) -> () {
    return false;
}
pub unsafe fn operator_lt_21(mut _a0: (), _a1: *const CustomEquality) -> () {
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CustomEquality {}
pub unsafe fn test_22() -> () {
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let nan: () = unimplemented!();
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
        let nan: () = unimplemented!();
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
        let nan: () = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StrongSpaceship_6 {
    pub value: (),
}
impl StrongSpaceship_6 {
    pub unsafe fn operator_eq(&self, _a0: *const StrongSpaceship_6) -> () {
        return (((*(self as *const StrongSpaceship_6)).value) == ((*_a0).value));
    }
    pub unsafe fn operator_cmp(&self, other: *const StrongSpaceship_6) -> () {
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
    pub value: (),
}
impl WeakSpaceship_8 {
    pub unsafe fn operator_eq(&self, _a0: *const WeakSpaceship_8) -> () {
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
    pub value: (),
}
impl PartialSpaceship_10 {
    pub unsafe fn operator_eq(&self, _a0: *const PartialSpaceship_10) -> () {
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
    pub value: (),
}
impl NoSpaceship_12 {
    pub unsafe fn operator_eq(&self, _a0: *const NoSpaceship_12) -> () {
        return (((*(self as *const NoSpaceship_12)).value) == ((*_a0).value));
    }
    pub unsafe fn operator_lt(&self, other: *const NoSpaceship_12) -> () {
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
    pub unsafe fn operator_cmp(&self, _a0: *const SpaceshipNoEquals_14) -> () {
        return unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_22() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
