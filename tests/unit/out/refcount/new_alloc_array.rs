extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut array: Ptr<i32> =
        Ptr::alloc_array((0..100_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    ({
        let ___s: AnyPtr = (array).to_any();
        memset_0(___s, 0, (4usize as usize).wrapping_mul(100_usize))
    });
    elem!(array, 99).write(-1_i32);
    let mut p1: Ptr<i32> = (array).clone();
    'loop_: while ((p1.read()) >= 0) {
        p1.write(1);
        p1.prefix_inc();
    }
    let mut out: i32 = 0;
    let mut p1: Ptr<i32> = (array).clone();
    'loop_: while ((p1.read()) >= 0) {
        out += { (p1.read()) };
        p1.prefix_inc();
    }
    let mut p2: Ptr<i32> = (array).clone();
    p2.delete();
    (if (out == 99) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"out == 99"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn memset_0(mut __s: AnyPtr, mut __c: i32, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
