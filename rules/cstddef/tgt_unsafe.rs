// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> u8 {
    Default::default()
}

fn t1002() -> usize {
    0_usize
}

fn t1003() -> isize {
    0_isize
}

fn t1005() -> usize {
    0_usize
}

fn t1006() -> isize {
    0_isize
}

fn f3<T1>(a0: u8, a1: T1) -> u8
where
    u8: ::std::ops::Shl<T1, Output = u8>,
{
    a0 << a1
}

fn f12<T1: FromByte>(a0: u8) -> T1 {
    <T1 as FromByte>::cpp_to_integer(a0)
}

unsafe fn f1(a0: u8, a1: u8) -> u8 {
    a0 & a1
}

unsafe fn f2(a0: *mut u8, a1: u8) -> *mut u8 {
    *a0 &= a1;
    a0
}

unsafe fn f4<T1>(a0: *mut u8, a1: T1) -> *mut u8
where
    u8: ::std::ops::Shl<T1, Output = u8>,
{
    *a0 = *a0 << a1;
    a0
}

fn f5<T1>(a0: u8, a1: T1) -> u8
where
    u8: ::std::ops::Shr<T1, Output = u8>,
{
    a0 >> a1
}

unsafe fn f6<T1>(a0: *mut u8, a1: T1) -> *mut u8
where
    u8: ::std::ops::Shr<T1, Output = u8>,
{
    *a0 = *a0 >> a1;
    a0
}

unsafe fn f7(a0: u8, a1: u8) -> u8 {
    a0 ^ a1
}

unsafe fn f8(a0: *mut u8, a1: u8) -> *mut u8 {
    *a0 ^= a1;
    a0
}

unsafe fn f9(a0: u8, a1: u8) -> u8 {
    a0 | a1
}

unsafe fn f10(a0: *mut u8, a1: u8) -> *mut u8 {
    *a0 |= a1;
    a0
}

unsafe fn f11(a0: u8) -> u8 {
    !a0
}
