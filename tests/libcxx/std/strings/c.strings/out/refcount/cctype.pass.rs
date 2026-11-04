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
pub struct test_macros_detail_is_same_int__int_ {}
impl CppMove for test_macros_detail_is_same_int__int_ {
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        ({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_alphanumeric()) {
                8
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_alphabetic()) {
                1024
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = ((' ' as i8) as i32);
            if (0..=0x7f).contains(&__c) && (__c == 0x20 || __c == 0x09) {
                1
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = ((' ' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_control()) {
                2
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_digit()) {
                2048
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_graphic()) {
                32768
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_lowercase()) {
                512
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((0x20..=0x7e).contains(&__c)) {
                16384
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_punctuation()) {
                4
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && (__c == 0x20 || (0x09..=0x0d).contains(&__c)) {
                8192
            } else {
                0
            }
        } != 0)
    );
    assert!(
        !({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_uppercase()) {
                256
            } else {
                0
            }
        } != 0)
    );
    assert!(
        ({
            let __c = (('a' as i8) as i32);
            if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_hexdigit()) {
                4096
            } else {
                0
            }
        } != 0)
    );
    assert!(
        (if (0..=255).contains(&(('A' as i8) as i32)) {
            ((('A' as i8) as i32) as u8).to_ascii_lowercase() as i32
        } else {
            (('A' as i8) as i32)
        } == (('a' as i8) as i32))
    );
    assert!(
        (if (0..=255).contains(&(('a' as i8) as i32)) {
            ((('a' as i8) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (('a' as i8) as i32)
        } == (('A' as i8) as i32))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
