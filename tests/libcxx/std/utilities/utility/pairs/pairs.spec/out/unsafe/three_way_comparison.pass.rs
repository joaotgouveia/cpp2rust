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
pub unsafe fn test_3() -> () {
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
        let nan: () = (unsafe { std_numeric_limits_double_::quiet_NaN() });
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
        let nan: () = (unsafe { std_numeric_limits_double_::quiet_NaN() });
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
            let nan: () = (unsafe { std_numeric_limits_double_::quiet_NaN() });
            unimplemented!();
        }
        {
            let nan: () = (unsafe { std_numeric_limits_double_::quiet_NaN() });
            unimplemented!();
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoSpaceship_4 {
    pub value: (),
}
impl NoSpaceship_4 {
    pub unsafe fn operator_eq(&self, _a0: *const NoSpaceship_4) -> () {
        return (((*(self as *const NoSpaceship_4)).value) == ((*_a0).value));
    }
    pub unsafe fn operator_lt(&self, other: *const NoSpaceship_4) -> () {
        return ((self.value) < ((*other).value));
    }
}
impl std::cmp::Ord for NoSpaceship_4 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if NoSpaceship_4::operator_lt(self, other as *const NoSpaceship_4) {
                std::cmp::Ordering::Less
            } else if NoSpaceship_4::operator_lt(other, self as *const NoSpaceship_4) {
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
        unsafe { NoSpaceship_4::operator_eq(self, other as *const NoSpaceship_4) }
    }
}
impl std::cmp::Eq for NoSpaceship_4 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoRelative_5 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoLessThan_6 {}
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
    (unsafe { test_3() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
