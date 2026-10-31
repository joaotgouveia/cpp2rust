extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_roundtrip_through_buffer_0(mut from: bool) {
    let mut middle: Buffer_1 = unimplemented!();
    let mut to: bool = unimplemented!();
    let mut middle2: Buffer_1 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut bool) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_1) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut bool) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_1) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_1) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_1) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_2(mut from: libc::c_char) {
    let mut middle: Buffer_3 = unimplemented!();
    let mut to: libc::c_char = unimplemented!();
    let mut middle2: Buffer_3 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_3) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_3) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_3) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_3) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_4(mut from: i8) {
    let mut middle: Buffer_5 = unimplemented!();
    let mut to: i8 = unimplemented!();
    let mut middle2: Buffer_5 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i8) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_5) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i8) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_5) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_5) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_5) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_6(mut from: i16) {
    let mut middle: Buffer_7 = unimplemented!();
    let mut to: i16 = unimplemented!();
    let mut middle2: Buffer_7 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i16) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_7) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i16) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_7) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_7) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_7) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_8(mut from: i32) {
    let mut middle: Buffer_9 = unimplemented!();
    let mut to: i32 = unimplemented!();
    let mut middle2: Buffer_9 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_9) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_9) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_9) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_9) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_10(mut from: i64) {
    let mut middle: Buffer_11 = unimplemented!();
    let mut to: i64 = unimplemented!();
    let mut middle2: Buffer_11 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_11) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_11) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_11) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_11) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_12(mut from: i64) {
    let mut middle: Buffer_13 = unimplemented!();
    let mut to: i64 = unimplemented!();
    let mut middle2: Buffer_13 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_13) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_13) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_13) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_13) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_14(mut from: u8) {
    let mut middle: Buffer_15 = unimplemented!();
    let mut to: u8 = unimplemented!();
    let mut middle2: Buffer_15 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u8) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_15) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u8) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_15) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_15) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_15) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_16(mut from: u16) {
    let mut middle: Buffer_17 = unimplemented!();
    let mut to: u16 = unimplemented!();
    let mut middle2: Buffer_17 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u16) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_17) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u16) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_17) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_17) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_17) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_18(mut from: u32) {
    let mut middle: Buffer_19 = unimplemented!();
    let mut to: u32 = unimplemented!();
    let mut middle2: Buffer_19 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_19) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_19) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_19) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_19) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_20(mut from: u64) {
    let mut middle: Buffer_21 = unimplemented!();
    let mut to: u64 = unimplemented!();
    let mut middle2: Buffer_21 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_21) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_21) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_21) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_21) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_22(mut from: u64) {
    let mut middle: Buffer_23 = unimplemented!();
    let mut to: u64 = unimplemented!();
    let mut middle2: Buffer_23 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_23) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_23) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_23) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_23) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_24(mut from: f32) {
    let mut middle: Buffer_25 = unimplemented!();
    let mut to: f32 = unimplemented!();
    let mut middle2: Buffer_25 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_25) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_25) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_25) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_25) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_26(mut from: f64) {
    let mut middle: Buffer_27 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: Buffer_27 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_27) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_27) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_27) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_27) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_28(mut from: f64) {
    let mut middle: Buffer_29 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: Buffer_29 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {}
}
pub unsafe fn test_roundtrip_through_buffer_30(mut from: *mut ::libc::c_void) {
    let mut middle: Buffer_31 = unimplemented!();
    let mut to: *mut ::libc::c_void = unimplemented!();
    let mut middle2: Buffer_31 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_31) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_31) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_31) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_31) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_buffer_32(mut from: *mut i32) {
    let mut middle: Buffer_33 = unimplemented!();
    let mut to: *mut i32 = unimplemented!();
    let mut middle2: Buffer_33 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_33) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_33) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Buffer_33) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Buffer_33) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_1 {
    pub buffer: [libc::c_char; 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_3 {
    pub buffer: [libc::c_char; 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_5 {
    pub buffer: [libc::c_char; 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_7 {
    pub buffer: [libc::c_char; 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_9 {
    pub buffer: [libc::c_char; 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_11 {
    pub buffer: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_13 {
    pub buffer: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_15 {
    pub buffer: [libc::c_char; 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_17 {
    pub buffer: [libc::c_char; 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_19 {
    pub buffer: [libc::c_char; 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_21 {
    pub buffer: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_23 {
    pub buffer: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_25 {
    pub buffer: [libc::c_char; 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_27 {
    pub buffer: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_29 {
    pub buffer: [libc::c_char; 16],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_31 {
    pub buffer: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Buffer_33 {
    pub buffer: [libc::c_char; 8],
}
pub unsafe fn test_roundtrip_through_nested_T_34(mut from: bool) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_35>()) == (::std::mem::size_of::<bool>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_35 = unimplemented!();
    let mut to: bool = unimplemented!();
    let mut middle2: Nested_35 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut bool) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_35) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut bool) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_35) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_35) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_35) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_36(mut from: libc::c_char) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_37>()) == (::std::mem::size_of::<libc::c_char>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_37 = unimplemented!();
    let mut to: libc::c_char = unimplemented!();
    let mut middle2: Nested_37 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_37) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_37) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_37) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_37) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<libc::c_char>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_38(mut from: i8) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_39>()) == (::std::mem::size_of::<i8>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_39 = unimplemented!();
    let mut to: i8 = unimplemented!();
    let mut middle2: Nested_39 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i8) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_39) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i8) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_39) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_39) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_39) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_40(mut from: i16) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_41>()) == (::std::mem::size_of::<i16>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_41 = unimplemented!();
    let mut to: i16 = unimplemented!();
    let mut middle2: Nested_41 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i16) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_41) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i16) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_41) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_41) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_41) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_42(mut from: i32) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_43>()) == (::std::mem::size_of::<i32>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_43 = unimplemented!();
    let mut to: i32 = unimplemented!();
    let mut middle2: Nested_43 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_43) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_43) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_43) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_43) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_44(mut from: i64) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_45>()) == (::std::mem::size_of::<i64>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_45 = unimplemented!();
    let mut to: i64 = unimplemented!();
    let mut middle2: Nested_45 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_45) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_45) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_45) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_45) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_46(mut from: i64) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_47>()) == (::std::mem::size_of::<i64>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_47 = unimplemented!();
    let mut to: i64 = unimplemented!();
    let mut middle2: Nested_47 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_47) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_47) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_47) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_47) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_48(mut from: u8) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_49>()) == (::std::mem::size_of::<u8>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_49 = unimplemented!();
    let mut to: u8 = unimplemented!();
    let mut middle2: Nested_49 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u8) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_49) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u8) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_49) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_49) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_49) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u8>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_50(mut from: u16) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_51>()) == (::std::mem::size_of::<u16>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_51 = unimplemented!();
    let mut to: u16 = unimplemented!();
    let mut middle2: Nested_51 = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u16) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_51) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u16) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_51) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_51) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_51) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u16>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_52(mut from: u32) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_53>()) == (::std::mem::size_of::<u32>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_53 = unimplemented!();
    let mut to: u32 = unimplemented!();
    let mut middle2: Nested_53 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_53) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_53) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_53) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_53) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_54(mut from: u64) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_55>()) == (::std::mem::size_of::<u64>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_55 = unimplemented!();
    let mut to: u64 = unimplemented!();
    let mut middle2: Nested_55 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_55) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_55) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_55) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_55) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_56(mut from: u64) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_57>()) == (::std::mem::size_of::<u64>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_57 = unimplemented!();
    let mut to: u64 = unimplemented!();
    let mut middle2: Nested_57 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_57) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_57) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_57) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_57) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_58(mut from: f32) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_59>()) == (::std::mem::size_of::<f32>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_59 = unimplemented!();
    let mut to: f32 = unimplemented!();
    let mut middle2: Nested_59 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_59) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_59) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_59) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_59) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_60(mut from: f64) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_61>()) == (::std::mem::size_of::<f64>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_61 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: Nested_61 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_61) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_61) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_61) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_61) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_62(mut from: f64) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_63>()) == (::std::mem::size_of::<f64>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_63 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: Nested_63 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {}
}
pub unsafe fn test_roundtrip_through_nested_T_64(mut from: *mut ::libc::c_void) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_65>()) == (::std::mem::size_of::<*mut ::libc::c_void>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_65 = unimplemented!();
    let mut to: *mut ::libc::c_void = unimplemented!();
    let mut middle2: Nested_65 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_65) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_65) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_65) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_65) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_nested_T_66(mut from: *mut i32) {
    const _: () = assert!(
        ((::std::mem::size_of::<Nested_67>()) == (::std::mem::size_of::<*mut i32>())),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_67 = unimplemented!();
    let mut to: *mut i32 = unimplemented!();
    let mut middle2: Nested_67 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_67) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_67) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut Nested_67) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut Nested_67) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_35 {
    pub x: bool,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_37 {
    pub x: libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_39 {
    pub x: i8,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_41 {
    pub x: i16,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_43 {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_45 {
    pub x: i64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_47 {
    pub x: i64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_49 {
    pub x: u8,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_51 {
    pub x: u16,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_53 {
    pub x: u32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_55 {
    pub x: u64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_57 {
    pub x: u64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_59 {
    pub x: f32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_61 {
    pub x: f64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_63 {
    pub x: f64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_65 {
    pub x: *mut ::libc::c_void,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_67 {
    pub x: *mut i32,
}
pub unsafe fn test_roundtrip_through_68(mut from: bool) {
    const _: () = assert!(
        ((::std::mem::size_of::<libc::c_char>()) == (::std::mem::size_of::<bool>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: libc::c_char = unimplemented!();
    let mut to: bool = unimplemented!();
    let mut middle2: libc::c_char = unimplemented!();
    assert!(((((from as i32) == (to as i32)) as i32) == (((from as i32) == (from as i32)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut bool) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut bool) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<bool>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_69(mut from: i32) {
    const _: () = assert!(
        ((::std::mem::size_of::<f32>()) == (::std::mem::size_of::<i32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: f32 = unimplemented!();
    let mut to: i32 = unimplemented!();
    let mut middle2: f32 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_70(mut from: i64) {
    const _: () = assert!(
        ((::std::mem::size_of::<f64>()) == (::std::mem::size_of::<i64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: f64 = unimplemented!();
    let mut to: i64 = unimplemented!();
    let mut middle2: f64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_71(mut from: u32) {
    const _: () = assert!(
        ((::std::mem::size_of::<f32>()) == (::std::mem::size_of::<u32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: f32 = unimplemented!();
    let mut to: u32 = unimplemented!();
    let mut middle2: f32 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_72(mut from: u64) {
    const _: () = assert!(
        ((::std::mem::size_of::<f64>()) == (::std::mem::size_of::<u64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: f64 = unimplemented!();
    let mut to: u64 = unimplemented!();
    let mut middle2: f64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_73(mut from: i32) {
    const _: () = assert!(
        ((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i32 = unimplemented!();
    let mut to: i32 = unimplemented!();
    let mut middle2: i32 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_74(mut from: i32) {
    const _: () = assert!(
        ((::std::mem::size_of::<u32>()) == (::std::mem::size_of::<i32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: u32 = unimplemented!();
    let mut to: i32 = unimplemented!();
    let mut middle2: u32 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_75(mut from: i64) {
    const _: () = assert!(
        ((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i64 = unimplemented!();
    let mut to: i64 = unimplemented!();
    let mut middle2: i64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_76(mut from: i64) {
    const _: () = assert!(
        ((::std::mem::size_of::<u64>()) == (::std::mem::size_of::<i64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: u64 = unimplemented!();
    let mut to: i64 = unimplemented!();
    let mut middle2: u64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_77(mut from: i64) {
    const _: () = assert!(
        ((::std::mem::size_of::<f64>()) == (::std::mem::size_of::<i64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: f64 = unimplemented!();
    let mut to: i64 = unimplemented!();
    let mut middle2: f64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<i64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_78(mut from: u32) {
    const _: () = assert!(
        ((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<u32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i32 = unimplemented!();
    let mut to: u32 = unimplemented!();
    let mut middle2: i32 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_79(mut from: u64) {
    const _: () = assert!(
        ((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<u64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i64 = unimplemented!();
    let mut to: u64 = unimplemented!();
    let mut middle2: i64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_80(mut from: u64) {
    const _: () = assert!(
        ((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<u64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i64 = unimplemented!();
    let mut to: u64 = unimplemented!();
    let mut middle2: i64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_81(mut from: u64) {
    const _: () = assert!(
        ((::std::mem::size_of::<f64>()) == (::std::mem::size_of::<u64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: f64 = unimplemented!();
    let mut to: u64 = unimplemented!();
    let mut middle2: f64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut u64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<u64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_82(mut from: f32) {
    const _: () = assert!(
        ((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<f32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i32 = unimplemented!();
    let mut to: f32 = unimplemented!();
    let mut middle2: i32 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut f32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_83(mut from: f64) {
    const _: () = assert!(
        ((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<f64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i64 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: i64 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut f64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut i64) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<f64>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_84(mut from: f64) {
    const _: () = assert!(
        ((::std::mem::size_of::<i128>()) == (::std::mem::size_of::<f64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i128 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: i128 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {}
}
pub unsafe fn test_roundtrip_through_85(mut from: f64) {
    const _: () = assert!(
        ((::std::mem::size_of::<u128>()) == (::std::mem::size_of::<f64>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: u128 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: u128 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {}
}
pub unsafe fn test_roundtrip_through_86(mut from: *mut ::libc::c_void) {
    const _: () = assert!(
        ((::std::mem::size_of::<*mut ::libc::c_void>())
            == (::std::mem::size_of::<*mut ::libc::c_void>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: *mut ::libc::c_void = unimplemented!();
    let mut to: *mut ::libc::c_void = unimplemented!();
    let mut middle2: *mut ::libc::c_void = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut ::libc::c_void) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut ::libc::c_void) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut ::libc::c_void) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut *mut ::libc::c_void) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_87(mut from: *mut ::libc::c_void) {
    const _: () = assert!(
        ((::std::mem::size_of::<*mut libc::c_char>())
            == (::std::mem::size_of::<*mut ::libc::c_void>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: *mut libc::c_char = unimplemented!();
    let mut to: *mut ::libc::c_void = unimplemented!();
    let mut middle2: *mut libc::c_char = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut *mut libc::c_char) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_88(mut from: *mut ::libc::c_void) {
    const _: () = assert!(
        ((::std::mem::size_of::<*mut i32>()) == (::std::mem::size_of::<*mut ::libc::c_void>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: *mut i32 = unimplemented!();
    let mut to: *mut ::libc::c_void = unimplemented!();
    let mut middle2: *mut i32 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut ::libc::c_void) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_89(mut from: *mut i32) {
    const _: () = assert!(
        ((::std::mem::size_of::<*mut i32>()) == (::std::mem::size_of::<*mut i32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: *mut i32 = unimplemented!();
    let mut to: *mut i32 = unimplemented!();
    let mut middle2: *mut i32 = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_90(mut from: *mut i32) {
    const _: () = assert!(
        ((::std::mem::size_of::<*mut libc::c_char>()) == (::std::mem::size_of::<*mut i32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: *mut libc::c_char = unimplemented!();
    let mut to: *mut i32 = unimplemented!();
    let mut middle2: *mut libc::c_char = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut libc::c_char) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut *mut libc::c_char) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn test_roundtrip_through_91(mut from: *mut i32) {
    const _: () = assert!(
        ((::std::mem::size_of::<*mut ::libc::c_void>()) == (::std::mem::size_of::<*mut i32>())),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: *mut ::libc::c_void = unimplemented!();
    let mut to: *mut i32 = unimplemented!();
    let mut middle2: *mut ::libc::c_void = unimplemented!();
    assert!(((((from) == (to)) as i32) == (((from) == (from)) as i32)));
    {
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut from as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut ::libc::c_void) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut to as *mut *mut i32) as *const ::libc::c_void) as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut ::libc::c_void) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    ((&mut middle as *mut *mut ::libc::c_void) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&mut middle2 as *mut *mut ::libc::c_void) as *const ::libc::c_void)
                        as *const u8,
                    ::std::mem::size_of::<*mut i32>() as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
}
pub unsafe fn generate_signed_integral_values_92() -> Vec<i8> {
    return Vec::from([
        i8::MIN,
        (((i8::MIN as i32) + (1)) as i8),
        (-2_i32 as i8),
        (-1_i32 as i8),
        0_i8,
        1_i8,
        2_i8,
        3_i8,
        (((i8::MAX as i32) - (1)) as i8),
        i8::MAX,
    ]);
}
pub unsafe fn generate_signed_integral_values_93() -> Vec<i16> {
    return Vec::from([
        i16::MIN,
        (((i16::MIN as i32) + (1)) as i16),
        (-2_i32 as i16),
        (-1_i32 as i16),
        0_i16,
        1_i16,
        2_i16,
        3_i16,
        (((i16::MAX as i32) - (1)) as i16),
        i16::MAX,
    ]);
}
pub unsafe fn generate_signed_integral_values_94() -> Vec<i32> {
    return Vec::from([
        i32::MIN,
        ((i32::MIN) + (1)),
        (-2_i32 as i32),
        (-1_i32 as i32),
        (0 as i32),
        (1 as i32),
        (2 as i32),
        (3 as i32),
        ((i32::MAX) - (1)),
        i32::MAX,
    ]);
}
pub unsafe fn generate_signed_integral_values_95() -> Vec<i64> {
    return Vec::from([
        i64::MIN,
        ((i64::MIN) + (1_i64)),
        (-2_i32 as i64),
        (-1_i32 as i64),
        0_i64,
        1_i64,
        2_i64,
        3_i64,
        ((i64::MAX) - (1_i64)),
        i64::MAX,
    ]);
}
pub unsafe fn generate_signed_integral_values_96() -> Vec<i64> {
    return Vec::from([
        i64::MIN,
        ((i64::MIN) + (1_i64)),
        (-2_i32 as i64),
        (-1_i32 as i64),
        0_i64,
        1_i64,
        2_i64,
        3_i64,
        ((i64::MAX) - (1_i64)),
        i64::MAX,
    ]);
}
pub unsafe fn generate_unsigned_integral_values_97() -> Vec<u8> {
    return Vec::from([
        0_u8,
        1_u8,
        2_u8,
        3_u8,
        (((u8::MAX as i32) - (1)) as u8),
        u8::MAX,
    ]);
}
pub unsafe fn generate_unsigned_integral_values_98() -> Vec<u16> {
    return Vec::from([
        0_u16,
        1_u16,
        2_u16,
        3_u16,
        (((u16::MAX as i32) - (1)) as u16),
        u16::MAX,
    ]);
}
pub unsafe fn generate_unsigned_integral_values_99() -> Vec<u32> {
    return Vec::from([
        0_u32,
        1_u32,
        2_u32,
        3_u32,
        (u32::MAX as u32).wrapping_sub(1_u32),
        u32::MAX,
    ]);
}
pub unsafe fn generate_unsigned_integral_values_100() -> Vec<u64> {
    return Vec::from([
        0_u64,
        1_u64,
        2_u64,
        3_u64,
        (u64::MAX as u64).wrapping_sub(1_u64),
        u64::MAX,
    ]);
}
pub unsafe fn generate_unsigned_integral_values_101() -> Vec<u64> {
    return Vec::from([
        0_u64,
        1_u64,
        2_u64,
        3_u64,
        (unimplemented!() as u64).wrapping_sub(1_u64),
        unimplemented!(),
    ]);
}
pub unsafe fn tests_102() -> bool {
    'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut b: bool = (*b);
        (unsafe { test_roundtrip_through_nested_T_34(b) });
        (unsafe { test_roundtrip_through_buffer_0(b) });
        (unsafe { test_roundtrip_through_68(b) });
    }
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: libc::c_char = (*c);
        (unsafe { test_roundtrip_through_nested_T_36(c) });
        (unsafe { test_roundtrip_through_buffer_2(c) });
    }
    let mut __tmp_0: Vec<i8> = (unsafe { generate_signed_integral_values_92() });
    let __range1: *mut Vec<i8> = &mut __tmp_0;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i8 = (*i);
        (unsafe { test_roundtrip_through_nested_T_38(i) });
        (unsafe { test_roundtrip_through_buffer_4(i) });
    }
    let mut __tmp_1: Vec<i16> = (unsafe { generate_signed_integral_values_93() });
    let __range1: *mut Vec<i16> = &mut __tmp_1;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i16 = (*i);
        (unsafe { test_roundtrip_through_nested_T_40(i) });
        (unsafe { test_roundtrip_through_buffer_6(i) });
    }
    let mut __tmp_2: Vec<i32> = (unsafe { generate_signed_integral_values_94() });
    let __range1: *mut Vec<i32> = &mut __tmp_2;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i32 = (*i);
        (unsafe { test_roundtrip_through_nested_T_42(i) });
        (unsafe { test_roundtrip_through_buffer_8(i) });
        (unsafe { test_roundtrip_through_69(i) });
    }
    let mut __tmp_3: Vec<i64> = (unsafe { generate_signed_integral_values_95() });
    let __range1: *mut Vec<i64> = &mut __tmp_3;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i64 = (*i);
        (unsafe { test_roundtrip_through_nested_T_44(i) });
        (unsafe { test_roundtrip_through_buffer_10(i) });
    }
    let mut __tmp_4: Vec<i64> = (unsafe { generate_signed_integral_values_96() });
    let __range1: *mut Vec<i64> = &mut __tmp_4;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i64 = (*i);
        (unsafe { test_roundtrip_through_nested_T_46(i) });
        (unsafe { test_roundtrip_through_buffer_12(i) });
        (unsafe { test_roundtrip_through_70(i) });
    }
    let mut __tmp_5: Vec<u8> = (unsafe { generate_unsigned_integral_values_97() });
    let __range1: *mut Vec<u8> = &mut __tmp_5;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u8 = (*i);
        (unsafe { test_roundtrip_through_nested_T_48(i) });
        (unsafe { test_roundtrip_through_buffer_14(i) });
    }
    let mut __tmp_6: Vec<u16> = (unsafe { generate_unsigned_integral_values_98() });
    let __range1: *mut Vec<u16> = &mut __tmp_6;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u16 = (*i);
        (unsafe { test_roundtrip_through_nested_T_50(i) });
        (unsafe { test_roundtrip_through_buffer_16(i) });
    }
    let mut __tmp_7: Vec<u32> = (unsafe { generate_unsigned_integral_values_99() });
    let __range1: *mut Vec<u32> = &mut __tmp_7;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u32 = (*i);
        (unsafe { test_roundtrip_through_nested_T_52(i) });
        (unsafe { test_roundtrip_through_buffer_18(i) });
        (unsafe { test_roundtrip_through_71(i) });
    }
    let mut __tmp_8: Vec<u64> = (unsafe { generate_unsigned_integral_values_100() });
    let __range1: *mut Vec<u64> = &mut __tmp_8;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u64 = (*i);
        (unsafe { test_roundtrip_through_nested_T_54(i) });
        (unsafe { test_roundtrip_through_buffer_20(i) });
    }
    let mut __tmp_9: Vec<u64> = (unsafe { generate_unsigned_integral_values_101() });
    let __range1: *mut Vec<u64> = &mut __tmp_9;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u64 = (*i);
        (unsafe { test_roundtrip_through_nested_T_56(i) });
        (unsafe { test_roundtrip_through_buffer_22(i) });
        (unsafe { test_roundtrip_through_72(i) });
    }
    let mut __tmp_10: Vec<i32> = (unsafe { generate_signed_integral_values_94() });
    let __range1: *mut Vec<i32> = &mut __tmp_10;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i32 = (*i);
        (unsafe { test_roundtrip_through_nested_T_42(i) });
        (unsafe { test_roundtrip_through_buffer_8(i) });
        (unsafe { test_roundtrip_through_73(i) });
        (unsafe { test_roundtrip_through_74(i) });
        (unsafe { test_roundtrip_through_69(i) });
    }
    let mut __tmp_11: Vec<i64> = (unsafe { generate_signed_integral_values_95() });
    let __range1: *mut Vec<i64> = &mut __tmp_11;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i64 = (*i);
        (unsafe { test_roundtrip_through_nested_T_44(i) });
        (unsafe { test_roundtrip_through_buffer_10(i) });
        (unsafe { test_roundtrip_through_75(i) });
        (unsafe { test_roundtrip_through_76(i) });
        (unsafe { test_roundtrip_through_77(i) });
    }
    let mut __tmp_12: Vec<u32> = (unsafe { generate_unsigned_integral_values_99() });
    let __range1: *mut Vec<u32> = &mut __tmp_12;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u32 = (*i);
        (unsafe { test_roundtrip_through_nested_T_52(i) });
        (unsafe { test_roundtrip_through_buffer_18(i) });
        (unsafe { test_roundtrip_through_78(i) });
        (unsafe { test_roundtrip_through_78(i) });
        (unsafe { test_roundtrip_through_71(i) });
    }
    let mut __tmp_13: Vec<u64> = (unsafe { generate_unsigned_integral_values_100() });
    let __range1: *mut Vec<u64> = &mut __tmp_13;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u64 = (*i);
        (unsafe { test_roundtrip_through_nested_T_54(i) });
        (unsafe { test_roundtrip_through_buffer_20(i) });
        (unsafe { test_roundtrip_through_79(i) });
        (unsafe { test_roundtrip_through_80(i) });
        (unsafe { test_roundtrip_through_81(i) });
    }
    let mut __tmp_14: Vec<f32> = Vec::from([
        0_f32,
        1_f32,
        -1_f32,
        10_f32,
        -10_f32,
        1e+10_f32,
        1e-10_f32,
        1e+20_f32,
        1e-20_f32,
        2.71828_f32,
        3.14159_f32,
        f32::NAN,
        f32::NAN,
        unimplemented!(),
        unimplemented!(),
    ]);
    let __range1: *mut Vec<f32> = &mut __tmp_14;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: f32 = (*i);
        (unsafe { test_roundtrip_through_nested_T_58(i) });
        (unsafe { test_roundtrip_through_buffer_24(i) });
        (unsafe { test_roundtrip_through_82(i) });
    }
    let mut __tmp_15: Vec<f64> = Vec::from([
        0_f64,
        1_f64,
        -1_f64,
        10_f64,
        -10_f64,
        1e+10_f64,
        1e-10_f64,
        1e+100_f64,
        1e-100_f64,
        2.718281828459045_f64,
        3.141592653589793_f64,
        f64::NAN,
        unimplemented!(),
        unimplemented!(),
    ]);
    let __range1: *mut Vec<f64> = &mut __tmp_15;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: f64 = (*i);
        (unsafe { test_roundtrip_through_nested_T_60(i) });
        (unsafe { test_roundtrip_through_buffer_26(i) });
        (unsafe { test_roundtrip_through_83(i) });
    }
    let mut __tmp_16: Vec<f64> = Vec::from([
        0_f64,
        1_f64,
        -1_f64,
        10_f64,
        -10_f64,
        1e+10_f64,
        1e-10_f64,
        1e+100_f64,
        1e-100_f64,
        2.718281828459045_f64,
        3.141592653589793_f64,
        f64::NAN,
        unimplemented!(),
        unimplemented!(),
    ]);
    let __range1: *mut Vec<f64> = &mut __tmp_16;
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: f64 = (*i);
        (unsafe { test_roundtrip_through_nested_T_62(i) });
        (unsafe { test_roundtrip_through_buffer_28(i) });
        (unsafe { test_roundtrip_through_84(i) });
        (unsafe { test_roundtrip_through_85(i) });
    }
    {
        {
            let mut obj: i32 = 3;
            let mut p: *mut ::libc::c_void = ((&mut obj as *mut i32) as *mut ::libc::c_void);
            (unsafe { test_roundtrip_through_nested_T_64(p) });
            (unsafe { test_roundtrip_through_buffer_30(p) });
            (unsafe { test_roundtrip_through_86(p) });
            (unsafe { test_roundtrip_through_87(p) });
            (unsafe { test_roundtrip_through_88(p) });
        }
        {
            let mut obj: i32 = 3;
            let mut p: *mut i32 = (&mut obj as *mut i32);
            (unsafe { test_roundtrip_through_nested_T_66(p) });
            (unsafe { test_roundtrip_through_buffer_32(p) });
            (unsafe { test_roundtrip_through_89(p) });
            (unsafe { test_roundtrip_through_90(p) });
            (unsafe { test_roundtrip_through_91(p) });
        }
    }
    return true;
}
pub unsafe fn basic_constexpr_test_103() -> bool {
    let mut from: i32 = 3;
    let mut middle: Nested_104 = unimplemented!();
    let mut to: i32 = unimplemented!();
    assert!(((from) == (to)));
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested_104 {
    pub buffer: [libc::c_char; 4],
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
    (unsafe { tests_102() });
    const _: () = assert!(true, "basic_constexpr_test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
