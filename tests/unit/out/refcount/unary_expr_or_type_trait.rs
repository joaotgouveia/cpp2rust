extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub c: i8,
    #[offset(8)]
    pub x: i64,
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn pack_size_0() -> u64 {
    return ((0 as u64).wrapping_add((0 as u64)) as u64);
}
pub fn pack_size_1(mut args_0: i32, mut args_1: f64) -> u64 {
    return ((2 as u64).wrapping_add((2 as u64)) as u64);
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
    ({ cpp2rust_assert_fail_4((::std::mem::size_of::<i32>() == 4_u64)) });
    ({ cpp2rust_assert_fail_4((::std::mem::size_of::<[i64; 4]>() == 32_u64)) });
    ({ cpp2rust_assert_fail_4((16usize == 16_u64)) });
    ({ cpp2rust_assert_fail_4((::std::mem::align_of::<i32>() == 4_u64)) });
    ({ cpp2rust_assert_fail_4((16usize == 16_u64)) });
    ({ cpp2rust_assert_fail_4((::std::mem::align_of::<[i64; 4]>() == 8_u64)) });
    ({ cpp2rust_assert_fail_4((16usize == 16_u64)) });
    ({ cpp2rust_assert_fail_4((({ pack_size_0() }) == 0_u64)) });
    ({ cpp2rust_assert_fail_4((({ pack_size_1(1, 2_f64) }) == 4_u64)) });
    ({ cpp2rust_assert_fail_4(true) });
    ({ cpp2rust_assert_fail_4(!(false)) });
    let mut may: bool = false;
    ({ cpp2rust_assert_fail_4(!(may)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
