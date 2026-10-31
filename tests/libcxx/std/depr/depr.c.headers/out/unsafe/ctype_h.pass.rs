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
pub struct test_macros_detail_is_same_int__int_ {}
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
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
    assert!(
        ({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_alphanumeric()) {
                8
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_alphabetic()) {
                1024
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = ((' ' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && (__c == 0x20 || __c == 0x09) {
                1
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = ((' ' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_control()) {
                2
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_digit()) {
                2048
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_graphic()) {
                32768
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_lowercase()) {
                512
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((0x20..=0x7e).contains(&__c)) {
                16384
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_punctuation()) {
                4
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && (__c == 0x20 || (0x09..=0x0d).contains(&__c)) {
                8192
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_uppercase()) {
                256
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as libc::c_char) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_hexdigit()) {
                4096
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ((if (0..=255).contains(&(('A' as libc::c_char) as i32)) {
            ((('A' as libc::c_char) as i32) as u8).to_ascii_lowercase() as i32
        } else {
            (('A' as libc::c_char) as i32)
        }) == (('a' as libc::c_char) as i32))
    );
    assert!(
        ((if (0..=255).contains(&(('a' as libc::c_char) as i32)) {
            ((('a' as libc::c_char) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (('a' as libc::c_char) as i32)
        }) == (('A' as libc::c_char) as i32))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
