extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub c: i8,
    #[offset(8)]
    pub x: i64,
}
pub fn pack_size_0() -> u64 {
    return ((0 as usize).wrapping_add((0 as usize)) as u64);
}
pub fn pack_size_1(mut args_0: i32, mut args_1: f64) -> u64 {
    return ((2 as usize).wrapping_add((2 as usize)) as u64);
}
pub fn may_throw_2() {}
pub fn no_throw_3() {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut arr: [i64; 4] = [0_i64, 0_i64, 0_i64, 0_i64];
    let mut s: S = <S>::default();
    (if (4usize == 4_usize) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"sizeof(int32_t) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (32usize == 32_usize) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"sizeof(arr) == 32"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (16usize == 16_usize) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"sizeof(s) == 16"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (::std::mem::align_of::<i32>() == 4_usize) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"alignof(int32_t) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (16usize == 16_usize) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"alignof(S) == 16"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (::std::mem::align_of::<[i64; 4]>() == 8_usize) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"__alignof(arr) == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (16usize == 16_usize) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"__alignof(s) == 16"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ pack_size_0() }) == 0_u64) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"pack_size() == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ pack_size_1(1, 2.0E+0) }) == 4_u64) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"pack_size(1, 2.0) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if true {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"noexcept(no_throw())"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !(false) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"!noexcept(may_throw())"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut may: bool = false;
    (if !(may) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"!may"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_4(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
