extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_int_ {}
impl detail_parse_integer_impl_int_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_long_ {}
impl detail_parse_integer_impl_long_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_long_long_ {}
impl detail_parse_integer_impl_long_long_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_unsigned_int_ {}
impl detail_parse_integer_impl_unsigned_int_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_unsigned_long_ {}
impl detail_parse_integer_impl_unsigned_long_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_unsigned_long_long_ {}
impl detail_parse_integer_impl_unsigned_long_long_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const ()) -> () {
        return unimplemented!();
    }
}
pub unsafe fn parse_integer_0(str: *const ()) -> () {
    return (unsafe {
        detail_parse_integer_impl_int_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_int_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_1(str: *const ()) -> () {
    return (unsafe {
        detail_parse_integer_impl_long_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_long_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_2(str: *const ()) -> () {
    return (unsafe {
        detail_parse_integer_impl_long_long_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_long_long_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_3(str: *const ()) -> () {
    return (unsafe {
        detail_parse_integer_impl_unsigned_int_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_unsigned_int_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_4(str: *const ()) -> () {
    return (unsafe {
        detail_parse_integer_impl_unsigned_long_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_unsigned_long_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_5(str: *const ()) -> () {
    return (unsafe {
        detail_parse_integer_impl_unsigned_long_long_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_unsigned_long_long_>::default(),
            str,
        )
    });
}
pub unsafe fn test_signed_6() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut t: () = (unsafe { parse_integer_0(&s) });
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        let mut t: () = (unsafe { parse_integer_0(&s) });
        unimplemented!();
    }
}
pub unsafe fn test_signed_7() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut t: () = (unsafe { parse_integer_1(&s) });
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        let mut t: () = (unsafe { parse_integer_1(&s) });
        unimplemented!();
    }
}
pub unsafe fn test_signed_8() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut t: () = (unsafe { parse_integer_2(&s) });
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        let mut t: () = (unsafe { parse_integer_2(&s) });
        unimplemented!();
    }
}
pub unsafe fn test_unsigned_9() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut t: () = (unsafe { parse_integer_3(&s) });
        unimplemented!();
    }
}
pub unsafe fn test_unsigned_10() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut t: () = (unsafe { parse_integer_4(&s) });
        unimplemented!();
    }
}
pub unsafe fn test_unsigned_11() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut t: () = (unsafe { parse_integer_5(&s) });
        unimplemented!();
    }
}
pub unsafe fn test_float_12() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_float_13() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_float_14() {
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
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
    (unsafe { test_signed_6() });
    (unsafe { test_signed_7() });
    (unsafe { test_signed_8() });
    (unsafe { test_unsigned_9() });
    (unsafe { test_unsigned_10() });
    (unsafe { test_unsigned_11() });
    (unsafe { test_float_12() });
    (unsafe { test_float_13() });
    (unsafe { test_float_14() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
