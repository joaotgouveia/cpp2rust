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
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const Vec<i32>) -> i32 {
        return unimplemented!();
    }
}
impl Callable1<*const Vec<i32>, i32> for detail_parse_integer_impl_int_ {
    fn call(&self, a1: *const Vec<i32>) -> i32 {
        unsafe { detail_parse_integer_impl_int_::operator_call_1_wchar_t(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_long_ {}
impl detail_parse_integer_impl_long_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const Vec<i32>) -> i64 {
        return unimplemented!();
    }
}
impl Callable1<*const Vec<i32>, i64> for detail_parse_integer_impl_long_ {
    fn call(&self, a1: *const Vec<i32>) -> i64 {
        unsafe { detail_parse_integer_impl_long_::operator_call_1_wchar_t(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_long_long_ {}
impl detail_parse_integer_impl_long_long_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const Vec<i32>) -> i64 {
        return unimplemented!();
    }
}
impl Callable1<*const Vec<i32>, i64> for detail_parse_integer_impl_long_long_ {
    fn call(&self, a1: *const Vec<i32>) -> i64 {
        unsafe { detail_parse_integer_impl_long_long_::operator_call_1_wchar_t(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_unsigned_int_ {}
impl detail_parse_integer_impl_unsigned_int_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const Vec<i32>) -> u32 {
        return (unimplemented!() as u32);
    }
}
impl Callable1<*const Vec<i32>, u32> for detail_parse_integer_impl_unsigned_int_ {
    fn call(&self, a1: *const Vec<i32>) -> u32 {
        unsafe { detail_parse_integer_impl_unsigned_int_::operator_call_1_wchar_t(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_unsigned_long_ {}
impl detail_parse_integer_impl_unsigned_long_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const Vec<i32>) -> u64 {
        return unimplemented!();
    }
}
impl Callable1<*const Vec<i32>, u64> for detail_parse_integer_impl_unsigned_long_ {
    fn call(&self, a1: *const Vec<i32>) -> u64 {
        unsafe { detail_parse_integer_impl_unsigned_long_::operator_call_1_wchar_t(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_parse_integer_impl_unsigned_long_long_ {}
impl detail_parse_integer_impl_unsigned_long_long_ {
    pub unsafe fn operator_call_1_wchar_t(&self, str: *const Vec<i32>) -> u64 {
        return unimplemented!();
    }
}
impl Callable1<*const Vec<i32>, u64> for detail_parse_integer_impl_unsigned_long_long_ {
    fn call(&self, a1: *const Vec<i32>) -> u64 {
        unsafe { detail_parse_integer_impl_unsigned_long_long_::operator_call_1_wchar_t(self, a1) }
    }
}
pub unsafe fn parse_integer_0(str: *const Vec<i32>) -> i32 {
    return (unsafe {
        detail_parse_integer_impl_int_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_int_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_1(str: *const Vec<i32>) -> i64 {
    return (unsafe {
        detail_parse_integer_impl_long_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_long_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_2(str: *const Vec<i32>) -> i64 {
    return (unsafe {
        detail_parse_integer_impl_long_long_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_long_long_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_3(str: *const Vec<i32>) -> u32 {
    return (unsafe {
        detail_parse_integer_impl_unsigned_int_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_unsigned_int_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_4(str: *const Vec<i32>) -> u64 {
    return (unsafe {
        detail_parse_integer_impl_unsigned_long_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_unsigned_long_>::default(),
            str,
        )
    });
}
pub unsafe fn parse_integer_5(str: *const Vec<i32>) -> u64 {
    return (unsafe {
        detail_parse_integer_impl_unsigned_long_long_::operator_call_1_wchar_t(
            &<detail_parse_integer_impl_unsigned_long_long_>::default(),
            str,
        )
    });
}
pub unsafe fn test_signed_6() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (1_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [48 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([48 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (5_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (6_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (((unimplemented!()) + (1)) as usize)));
        let mut t: i32 = (unsafe { parse_integer_0(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (i32::MAX)));
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        let mut t: i32 = (unsafe { parse_integer_0(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (i32::MIN)));
    }
}
pub unsafe fn test_signed_7() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (1_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [48 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([48 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (5_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (6_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (((unimplemented!()) + (1)) as usize)));
        let mut t: i64 = (unsafe { parse_integer_1(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (i64::MAX)));
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        let mut t: i64 = (unsafe { parse_integer_1(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (i64::MIN)));
    }
}
pub unsafe fn test_signed_8() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (1_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [48 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([48 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (5_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (6_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (((unimplemented!()) + (1)) as usize)));
        let mut t: i64 = (unsafe { parse_integer_2(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (i64::MAX)));
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        let mut t: i64 = (unsafe { parse_integer_2(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (i64::MIN)));
    }
}
pub unsafe fn test_unsigned_9() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (1_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [48 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([48 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (5_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (((unimplemented!()) + (1)) as usize)));
        let mut t: u32 = (unsafe { parse_integer_3(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (u32::MAX)));
    }
}
pub unsafe fn test_unsigned_10() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (1_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [48 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([48 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (5_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (((unimplemented!()) + (1)) as usize)));
        let mut t: u64 = (unsafe { parse_integer_4(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (u64::MAX)));
    }
}
pub unsafe fn test_unsigned_11() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (1_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [48 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([48 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (5_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (((unimplemented!()) + (1)) as usize)));
        let mut t: u64 = (unsafe { parse_integer_5(&*(&raw const s).cast::<Vec<i32>>()) });
        assert!(((t) == (unimplemented!())));
    }
}
pub unsafe fn test_float_12() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (8_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    48 as i32, 46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                    48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        48 as i32, 46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                        48 as i32, 48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (12_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32, 48 as i32,
                    48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32,
                        48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (13_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32,
                    48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32,
                        46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                        48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
}
pub unsafe fn test_float_13() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (8_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    48 as i32, 46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                    48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        48 as i32, 46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                        48 as i32, 48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (12_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32, 48 as i32,
                    48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32,
                        48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (13_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32,
                    48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32,
                        46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                        48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
}
pub unsafe fn test_float_14() {
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (8_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    48 as i32, 46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                    48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        48 as i32, 46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                        48 as i32, 48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (12_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32, 48 as i32,
                    48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32,
                        48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut s: Vec<i32> = unimplemented!();
        assert!(((s.len() - 1) == (13_usize)));
        assert!((((*(&mut s).as_mut_ptr().add((s.len() - 1) as usize)) as i32) == (0)));
        assert!(
            s == std::slice::from_raw_parts(
                [
                    45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 46 as i32,
                    48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        45 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32,
                        46 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32, 48 as i32,
                        48 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_signed_6() });
    (unsafe { test_signed_7() });
    (unsafe { test_signed_8() });
    (unsafe { test_unsigned_9() });
    (unsafe { test_unsigned_10() });
    (unsafe { test_unsigned_11() });
    (unsafe { test_float_12() });
    (unsafe { test_float_13() });
    (unsafe { test_float_14() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
